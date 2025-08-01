/// This was originally generated by collecting directives from ui tests and then extracting their
/// directive names. This is **not** an exhaustive list of all possible directives. Instead, this is
/// a best-effort approximation for diagnostics. Add new directives to this list when needed.
pub(crate) const KNOWN_DIRECTIVE_NAMES: &[&str] = &[
    // tidy-alphabetical-start
    "add-core-stubs",
    "assembly-output",
    "aux-bin",
    "aux-build",
    "aux-codegen-backend",
    "aux-crate",
    "build-aux-docs",
    "build-fail",
    "build-pass",
    "check-fail",
    "check-pass",
    "check-run-results",
    "check-stdout",
    "check-test-line-numbers-match",
    "compile-flags",
    "doc-flags",
    "dont-check-compiler-stderr",
    "dont-check-compiler-stdout",
    "dont-check-failure-status",
    "dont-require-annotations",
    "edition",
    "error-pattern",
    "exact-llvm-major-version",
    "exec-env",
    "failure-status",
    "filecheck-flags",
    "forbid-output",
    "force-host",
    "ignore-16bit",
    "ignore-32bit",
    "ignore-64bit",
    "ignore-aarch64",
    "ignore-aarch64-pc-windows-msvc",
    "ignore-aarch64-unknown-linux-gnu",
    "ignore-aix",
    "ignore-android",
    "ignore-apple",
    "ignore-arm",
    "ignore-arm-unknown-linux-gnueabi",
    "ignore-arm-unknown-linux-gnueabihf",
    "ignore-arm-unknown-linux-musleabi",
    "ignore-arm-unknown-linux-musleabihf",
    "ignore-auxiliary",
    "ignore-avr",
    "ignore-backends",
    "ignore-beta",
    "ignore-cdb",
    "ignore-compare-mode-next-solver",
    "ignore-compare-mode-polonius",
    "ignore-coverage-map",
    "ignore-coverage-run",
    "ignore-cross-compile",
    "ignore-eabi",
    "ignore-elf",
    "ignore-emscripten",
    "ignore-endian-big",
    "ignore-enzyme",
    "ignore-freebsd",
    "ignore-fuchsia",
    "ignore-gdb",
    "ignore-gdb-version",
    "ignore-gnu",
    "ignore-haiku",
    "ignore-horizon",
    "ignore-i686-pc-windows-gnu",
    "ignore-i686-pc-windows-msvc",
    "ignore-illumos",
    "ignore-ios",
    "ignore-linux",
    "ignore-lldb",
    "ignore-llvm-version",
    "ignore-loongarch32",
    "ignore-loongarch64",
    "ignore-macabi",
    "ignore-macos",
    "ignore-msp430",
    "ignore-msvc",
    "ignore-musl",
    "ignore-netbsd",
    "ignore-nightly",
    "ignore-none",
    "ignore-nto",
    "ignore-nvptx64",
    "ignore-nvptx64-nvidia-cuda",
    "ignore-openbsd",
    "ignore-pass",
    "ignore-powerpc",
    "ignore-powerpc64",
    "ignore-remote",
    "ignore-riscv64",
    "ignore-rustc-debug-assertions",
    "ignore-rustc_abi-x86-sse2",
    "ignore-s390x",
    "ignore-sgx",
    "ignore-sparc64",
    "ignore-spirv",
    "ignore-stable",
    "ignore-stage1",
    "ignore-stage2",
    "ignore-std-debug-assertions",
    "ignore-test",
    "ignore-thumb",
    "ignore-thumbv8m.base-none-eabi",
    "ignore-thumbv8m.main-none-eabi",
    "ignore-tvos",
    "ignore-unix",
    "ignore-unknown",
    "ignore-uwp",
    "ignore-visionos",
    "ignore-vxworks",
    "ignore-wasi",
    "ignore-wasm",
    "ignore-wasm32",
    "ignore-wasm32-bare",
    "ignore-wasm64",
    "ignore-watchos",
    "ignore-windows",
    "ignore-windows-gnu",
    "ignore-windows-msvc",
    "ignore-x32",
    "ignore-x86",
    "ignore-x86_64",
    "ignore-x86_64-apple-darwin",
    "ignore-x86_64-pc-windows-gnu",
    "ignore-x86_64-unknown-linux-gnu",
    "incremental",
    "known-bug",
    "llvm-cov-flags",
    "max-llvm-major-version",
    "min-cdb-version",
    "min-gdb-version",
    "min-lldb-version",
    "min-llvm-version",
    "min-system-llvm-version",
    "needs-asm-support",
    "needs-backends",
    "needs-crate-type",
    "needs-deterministic-layouts",
    "needs-dlltool",
    "needs-dynamic-linking",
    "needs-enzyme",
    "needs-force-clang-based-tests",
    "needs-git-hash",
    "needs-llvm-components",
    "needs-llvm-zstd",
    "needs-profiler-runtime",
    "needs-relocation-model-pic",
    "needs-run-enabled",
    "needs-rust-lld",
    "needs-rustc-debug-assertions",
    "needs-sanitizer-address",
    "needs-sanitizer-cfi",
    "needs-sanitizer-dataflow",
    "needs-sanitizer-hwaddress",
    "needs-sanitizer-kcfi",
    "needs-sanitizer-leak",
    "needs-sanitizer-memory",
    "needs-sanitizer-memtag",
    "needs-sanitizer-safestack",
    "needs-sanitizer-shadow-call-stack",
    "needs-sanitizer-support",
    "needs-sanitizer-thread",
    "needs-std-debug-assertions",
    "needs-subprocess",
    "needs-symlink",
    "needs-target-has-atomic",
    "needs-target-std",
    "needs-threads",
    "needs-unwind",
    "needs-wasmtime",
    "needs-xray",
    "no-auto-check-cfg",
    "no-prefer-dynamic",
    "normalize-stderr",
    "normalize-stderr-32bit",
    "normalize-stderr-64bit",
    "normalize-stdout",
    "only-16bit",
    "only-32bit",
    "only-64bit",
    "only-aarch64",
    "only-aarch64-apple-darwin",
    "only-aarch64-unknown-linux-gnu",
    "only-apple",
    "only-arm",
    "only-avr",
    "only-beta",
    "only-bpf",
    "only-cdb",
    "only-dist",
    "only-elf",
    "only-emscripten",
    "only-gnu",
    "only-i686-pc-windows-gnu",
    "only-i686-pc-windows-msvc",
    "only-i686-unknown-linux-gnu",
    "only-ios",
    "only-linux",
    "only-loongarch32",
    "only-loongarch64",
    "only-loongarch64-unknown-linux-gnu",
    "only-macos",
    "only-mips",
    "only-mips64",
    "only-msp430",
    "only-msvc",
    "only-musl",
    "only-nightly",
    "only-nvptx64",
    "only-powerpc",
    "only-riscv64",
    "only-rustc_abi-x86-sse2",
    "only-s390x",
    "only-sparc",
    "only-sparc64",
    "only-stable",
    "only-thumb",
    "only-tvos",
    "only-uefi",
    "only-unix",
    "only-visionos",
    "only-wasm32",
    "only-wasm32-bare",
    "only-wasm32-wasip1",
    "only-watchos",
    "only-windows",
    "only-windows-gnu",
    "only-windows-msvc",
    "only-x86",
    "only-x86_64",
    "only-x86_64-apple-darwin",
    "only-x86_64-fortanix-unknown-sgx",
    "only-x86_64-pc-windows-gnu",
    "only-x86_64-pc-windows-msvc",
    "only-x86_64-unknown-linux-gnu",
    "pp-exact",
    "pretty-compare-only",
    "pretty-mode",
    "proc-macro",
    "reference",
    "regex-error-pattern",
    "remap-src-base",
    "revisions",
    "run-crash",
    "run-fail",
    "run-fail-or-crash",
    "run-flags",
    "run-pass",
    "run-rustfix",
    "rustc-env",
    "rustfix-only-machine-applicable",
    "should-fail",
    "should-ice",
    "stderr-per-bitwidth",
    "test-mir-pass",
    "unique-doc-out-dir",
    "unset-exec-env",
    "unset-rustc-env",
    // Used by the tidy check `unknown_revision`.
    "unused-revision-names",
    // tidy-alphabetical-end
];

pub(crate) const KNOWN_HTMLDOCCK_DIRECTIVE_NAMES: &[&str] = &[
    "count",
    "!count",
    "files",
    "!files",
    "has",
    "!has",
    "has-dir",
    "!has-dir",
    "hasraw",
    "!hasraw",
    "matches",
    "!matches",
    "matchesraw",
    "!matchesraw",
    "snapshot",
    "!snapshot",
];

pub(crate) const KNOWN_JSONDOCCK_DIRECTIVE_NAMES: &[&str] =
    &["count", "!count", "has", "!has", "is", "!is", "ismany", "!ismany", "set", "!set"];
