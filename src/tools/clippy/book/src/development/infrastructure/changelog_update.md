# Changelog Update

If you want to help with updating the [changelog], you're in the right place.

## When to update

Typos and other small fixes/additions are _always_ welcome.

Special care needs to be taken when it comes to updating the changelog for a new
Rust release. For that purpose, the changelog is ideally updated during the week
before an upcoming stable release. You can find the release dates on the [Rust
Forge][forge].

Most of the time we only need to update the changelog for minor Rust releases.
It's been very rare that Clippy changes were included in a patch release.

## Changelog update walkthrough

### 1. Finding the relevant Clippy commits

Each Rust release ships with its own version of Clippy. The Clippy subtree can
be found in the `tools` directory of the Rust repository.

Depending on the current time and what exactly you want to update, the following
bullet points might be helpful:

* When writing the release notes for the **upcoming stable release** you need to
  check out the Clippy commit of the current Rust `beta` branch.
  [Link][rust_beta_tools]
* When writing the release notes for the **upcoming beta release**, you need to
  check out the Clippy commit of the current Rust `master`.
  [Link][rust_master_tools]
* When writing the (forgotten) release notes for a **past stable release**, you
  need to check out the Rust release tag of the stable release.
  [Link][rust_stable_tools]

Usually you want to write the changelog of the **upcoming stable release**. Make
sure though, that `beta` was already branched in the Rust repository.

To find the commit hash, issue the following command when in a `rust-lang/rust`
checkout (most of the time on the `upstream/beta` branch):
```
git log --oneline -- src/tools/clippy/ | grep -o "Merge commit '[a-f0-9]*' into .*" | head -1 | sed -e "s/Merge commit '\([a-f0-9]*\)' into .*/\1/g"
```

### 2. Fetching the PRs between those commits

Once you've got the correct commit range, run

```
util/fetch_prs_between.sh start_commit end_commit > changes.txt
```

where `end_commit` is the commit hash from the previous command and `start_commit`
is [the commit hash][beta_section] from the current CHANGELOG file.
Open `changes.txt` file in your editor of choice.

### 3. Authoring the final changelog

The above script should have dumped all the relevant PRs to the file you
specified. It should have filtered out most of the irrelevant PRs already, but
it's a good idea to do a manual cleanup pass and choose valuable PRs.
If you're not sure about some PRs, just leave them in for the review and
ask for feedback.

With the PRs filtered, you can start to take each PR and move the `changelog: `
content to `CHANGELOG.md`. Adapt the wording as you see fit but try to keep it
somewhat coherent.

The sections order should roughly be:

```
### New Lints
* Added [`LINT`] to `GROUP`

### Moves and Deprecations
* Moved [`LINT`] to `GROUP` (From `GROUP`, now LEVEL-by-default)
* Renamed `LINT` to [`LINT`]

### Enhancements
### False Positive Fixes
### ICE Fixes
### Documentation Improvements
### Others
```

Please also be sure to update [the `Unreleased/Beta/In Rust Nightly` section][beta_section] at the top with the
relevant commits ranges and to add the `Rust <version>` section with release date and PR ranges.

### 4. Include `beta-accepted` PRs

Look for the [`beta-accepted`] label and make sure to also include the PRs with
that label in the changelog. If you can, remove the `beta-accepted` labels
**after** the changelog PR was merged.

> _Note:_ Some of those PRs might even get backported to the previous `beta`.
> Those have to be included in the changelog of the _previous_ release.

### 5. Update `clippy::version` attributes

Next, make sure to check that the `#[clippy::version]` attributes for the added
lints contain the correct version. 
In order to find lints that need a version update, go through the lints in the 
"New Lints" section and run the following command for each lint name:

```
grep -rB1 "pub $LINT_NAME" .
```

The version shown should match the version of the release the changelog is 
written for. If not, update the version to the changelog version.

[changelog]: https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md
[forge]: https://forge.rust-lang.org/
[rust_master_tools]: https://github.com/rust-lang/rust/tree/master/src/tools/clippy
[rust_beta_tools]: https://github.com/rust-lang/rust/tree/beta/src/tools/clippy
[rust_stable_tools]: https://github.com/rust-lang/rust/releases
[`beta-accepted`]: https://github.com/rust-lang/rust-clippy/issues?q=label%3Abeta-accepted+
[beta_section]: https://github.com/rust-lang/rust-clippy/blob/master/CHANGELOG.md#unreleased--beta--in-rust-nightly
