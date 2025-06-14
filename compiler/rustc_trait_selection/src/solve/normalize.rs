use std::fmt::Debug;

use rustc_data_structures::stack::ensure_sufficient_stack;
use rustc_infer::infer::InferCtxt;
use rustc_infer::infer::at::At;
use rustc_infer::traits::solve::Goal;
use rustc_infer::traits::{FromSolverError, Obligation, TraitEngine};
use rustc_middle::traits::ObligationCause;
use rustc_middle::ty::{
    self, FallibleTypeFolder, Ty, TyCtxt, TypeFoldable, TypeFolder, TypeSuperFoldable,
    TypeVisitableExt, UniverseIndex,
};
use tracing::instrument;

use super::{FulfillmentCtxt, NextSolverError};
use crate::error_reporting::InferCtxtErrorExt;
use crate::error_reporting::traits::OverflowCause;
use crate::traits::{BoundVarReplacer, PlaceholderReplacer, ScrubbedTraitError};

/// Deeply normalize all aliases in `value`. This does not handle inference and expects
/// its input to be already fully resolved.
pub fn deeply_normalize<'tcx, T, E>(at: At<'_, 'tcx>, value: T) -> Result<T, Vec<E>>
where
    T: TypeFoldable<TyCtxt<'tcx>>,
    E: FromSolverError<'tcx, NextSolverError<'tcx>>,
{
    assert!(!value.has_escaping_bound_vars());
    deeply_normalize_with_skipped_universes(at, value, vec![])
}

/// Deeply normalize all aliases in `value`. This does not handle inference and expects
/// its input to be already fully resolved.
///
/// Additionally takes a list of universes which represents the binders which have been
/// entered before passing `value` to the function. This is currently needed for
/// `normalize_erasing_regions`, which skips binders as it walks through a type.
pub fn deeply_normalize_with_skipped_universes<'tcx, T, E>(
    at: At<'_, 'tcx>,
    value: T,
    universes: Vec<Option<UniverseIndex>>,
) -> Result<T, Vec<E>>
where
    T: TypeFoldable<TyCtxt<'tcx>>,
    E: FromSolverError<'tcx, NextSolverError<'tcx>>,
{
    let (value, coroutine_goals) =
        deeply_normalize_with_skipped_universes_and_ambiguous_coroutine_goals(
            at, value, universes,
        )?;
    assert_eq!(coroutine_goals, vec![]);

    Ok(value)
}

/// Deeply normalize all aliases in `value`. This does not handle inference and expects
/// its input to be already fully resolved.
///
/// Additionally takes a list of universes which represents the binders which have been
/// entered before passing `value` to the function. This is currently needed for
/// `normalize_erasing_regions`, which skips binders as it walks through a type.
///
/// This returns a set of stalled obligations involving coroutines if the typing mode of
/// the underlying infcx has any stalled coroutine def ids.
pub fn deeply_normalize_with_skipped_universes_and_ambiguous_coroutine_goals<'tcx, T, E>(
    at: At<'_, 'tcx>,
    value: T,
    universes: Vec<Option<UniverseIndex>>,
) -> Result<(T, Vec<Goal<'tcx, ty::Predicate<'tcx>>>), Vec<E>>
where
    T: TypeFoldable<TyCtxt<'tcx>>,
    E: FromSolverError<'tcx, NextSolverError<'tcx>>,
{
    let fulfill_cx = FulfillmentCtxt::new(at.infcx);
    let mut folder = NormalizationFolder {
        at,
        fulfill_cx,
        depth: 0,
        universes,
        stalled_coroutine_goals: vec![],
    };
    let value = value.try_fold_with(&mut folder)?;
    let errors = folder.fulfill_cx.select_all_or_error(at.infcx);
    if errors.is_empty() { Ok((value, folder.stalled_coroutine_goals)) } else { Err(errors) }
}

struct NormalizationFolder<'me, 'tcx, E> {
    at: At<'me, 'tcx>,
    fulfill_cx: FulfillmentCtxt<'tcx, E>,
    depth: usize,
    universes: Vec<Option<UniverseIndex>>,
    stalled_coroutine_goals: Vec<Goal<'tcx, ty::Predicate<'tcx>>>,
}

impl<'tcx, E> NormalizationFolder<'_, 'tcx, E>
where
    E: FromSolverError<'tcx, NextSolverError<'tcx>>,
{
    fn normalize_alias_term(
        &mut self,
        alias_term: ty::Term<'tcx>,
    ) -> Result<ty::Term<'tcx>, Vec<E>> {
        let infcx = self.at.infcx;
        let tcx = infcx.tcx;
        let recursion_limit = tcx.recursion_limit();
        if !recursion_limit.value_within_limit(self.depth) {
            let term = alias_term.to_alias_term().unwrap();

            self.at.infcx.err_ctxt().report_overflow_error(
                OverflowCause::DeeplyNormalize(term),
                self.at.cause.span,
                true,
                |_| {},
            );
        }

        self.depth += 1;

        let infer_term = infcx.next_term_var_of_kind(alias_term, self.at.cause.span);
        let obligation = Obligation::new(
            tcx,
            self.at.cause.clone(),
            self.at.param_env,
            ty::PredicateKind::AliasRelate(
                alias_term.into(),
                infer_term.into(),
                ty::AliasRelationDirection::Equate,
            ),
        );

        self.fulfill_cx.register_predicate_obligation(infcx, obligation);
        self.select_all_and_stall_coroutine_predicates()?;

        // Alias is guaranteed to be fully structurally resolved,
        // so we can super fold here.
        let term = infcx.resolve_vars_if_possible(infer_term);
        // super-folding the `term` will directly fold the `Ty` or `Const` so
        // we have to match on the term and super-fold them manually.
        let result = match term.kind() {
            ty::TermKind::Ty(ty) => ty.try_super_fold_with(self)?.into(),
            ty::TermKind::Const(ct) => ct.try_super_fold_with(self)?.into(),
        };
        self.depth -= 1;
        Ok(result)
    }

    fn select_all_and_stall_coroutine_predicates(&mut self) -> Result<(), Vec<E>> {
        let errors = self.fulfill_cx.select_where_possible(self.at.infcx);
        if !errors.is_empty() {
            return Err(errors);
        }

        self.stalled_coroutine_goals.extend(
            self.fulfill_cx
                .drain_stalled_obligations_for_coroutines(self.at.infcx)
                .into_iter()
                .map(|obl| obl.as_goal()),
        );

        let errors = self.fulfill_cx.collect_remaining_errors(self.at.infcx);
        if !errors.is_empty() {
            return Err(errors);
        }

        Ok(())
    }
}

impl<'tcx, E> FallibleTypeFolder<TyCtxt<'tcx>> for NormalizationFolder<'_, 'tcx, E>
where
    E: FromSolverError<'tcx, NextSolverError<'tcx>> + Debug,
{
    type Error = Vec<E>;

    fn cx(&self) -> TyCtxt<'tcx> {
        self.at.infcx.tcx
    }

    fn try_fold_binder<T: TypeFoldable<TyCtxt<'tcx>>>(
        &mut self,
        t: ty::Binder<'tcx, T>,
    ) -> Result<ty::Binder<'tcx, T>, Self::Error> {
        self.universes.push(None);
        let t = t.try_super_fold_with(self)?;
        self.universes.pop();
        Ok(t)
    }

    #[instrument(level = "trace", skip(self), ret)]
    fn try_fold_ty(&mut self, ty: Ty<'tcx>) -> Result<Ty<'tcx>, Self::Error> {
        let infcx = self.at.infcx;
        debug_assert_eq!(ty, infcx.shallow_resolve(ty));
        if !ty.has_aliases() {
            return Ok(ty);
        }

        let ty::Alias(..) = *ty.kind() else { return ty.try_super_fold_with(self) };

        if ty.has_escaping_bound_vars() {
            let (ty, mapped_regions, mapped_types, mapped_consts) =
                BoundVarReplacer::replace_bound_vars(infcx, &mut self.universes, ty);
            let result =
                ensure_sufficient_stack(|| self.normalize_alias_term(ty.into()))?.expect_type();
            Ok(PlaceholderReplacer::replace_placeholders(
                infcx,
                mapped_regions,
                mapped_types,
                mapped_consts,
                &self.universes,
                result,
            ))
        } else {
            Ok(ensure_sufficient_stack(|| self.normalize_alias_term(ty.into()))?.expect_type())
        }
    }

    #[instrument(level = "trace", skip(self), ret)]
    fn try_fold_const(&mut self, ct: ty::Const<'tcx>) -> Result<ty::Const<'tcx>, Self::Error> {
        let infcx = self.at.infcx;
        debug_assert_eq!(ct, infcx.shallow_resolve_const(ct));
        if !ct.has_aliases() {
            return Ok(ct);
        }

        let ty::ConstKind::Unevaluated(..) = ct.kind() else { return ct.try_super_fold_with(self) };

        if ct.has_escaping_bound_vars() {
            let (ct, mapped_regions, mapped_types, mapped_consts) =
                BoundVarReplacer::replace_bound_vars(infcx, &mut self.universes, ct);
            let result =
                ensure_sufficient_stack(|| self.normalize_alias_term(ct.into()))?.expect_const();
            Ok(PlaceholderReplacer::replace_placeholders(
                infcx,
                mapped_regions,
                mapped_types,
                mapped_consts,
                &self.universes,
                result,
            ))
        } else {
            Ok(ensure_sufficient_stack(|| self.normalize_alias_term(ct.into()))?.expect_const())
        }
    }
}

// Deeply normalize a value and return it
pub(crate) fn deeply_normalize_for_diagnostics<'tcx, T: TypeFoldable<TyCtxt<'tcx>>>(
    infcx: &InferCtxt<'tcx>,
    param_env: ty::ParamEnv<'tcx>,
    t: T,
) -> T {
    t.fold_with(&mut DeeplyNormalizeForDiagnosticsFolder {
        at: infcx.at(&ObligationCause::dummy(), param_env),
    })
}

struct DeeplyNormalizeForDiagnosticsFolder<'a, 'tcx> {
    at: At<'a, 'tcx>,
}

impl<'tcx> TypeFolder<TyCtxt<'tcx>> for DeeplyNormalizeForDiagnosticsFolder<'_, 'tcx> {
    fn cx(&self) -> TyCtxt<'tcx> {
        self.at.infcx.tcx
    }

    fn fold_ty(&mut self, ty: Ty<'tcx>) -> Ty<'tcx> {
        let infcx = self.at.infcx;
        let result: Result<_, Vec<ScrubbedTraitError<'tcx>>> = infcx.commit_if_ok(|_| {
            deeply_normalize_with_skipped_universes_and_ambiguous_coroutine_goals(
                self.at,
                ty,
                vec![None; ty.outer_exclusive_binder().as_usize()],
            )
        });
        match result {
            Ok((ty, _)) => ty,
            Err(_) => ty.super_fold_with(self),
        }
    }

    fn fold_const(&mut self, ct: ty::Const<'tcx>) -> ty::Const<'tcx> {
        let infcx = self.at.infcx;
        let result: Result<_, Vec<ScrubbedTraitError<'tcx>>> = infcx.commit_if_ok(|_| {
            deeply_normalize_with_skipped_universes_and_ambiguous_coroutine_goals(
                self.at,
                ct,
                vec![None; ct.outer_exclusive_binder().as_usize()],
            )
        });
        match result {
            Ok((ct, _)) => ct,
            Err(_) => ct.super_fold_with(self),
        }
    }
}
