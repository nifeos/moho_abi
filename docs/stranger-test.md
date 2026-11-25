# The stranger cadence

The release's whole claim is that a stranger, given this repository and nothing else, can build
the crate and run its proofs from the README alone. A real agent runs that check on a
cadence, and the weekly `stranger cadence` job fails when the
last run is past 28 days.

What a run is:

1. Clone the repository fresh.
2. Follow the README and nothing else: build, test, and run the proofs.
   The README must name the build and test commands (with any feature a test target needs), the
   bare-metal build with `rustup target add`, the Kani install before the first `cargo kani`, how
   long the proofs take and how to run one harness, what a passing run prints that looks alarming,
   how to run and revert one falsification patch, and how to check the minimum Rust version.
   Each one it lacks is a finding.
3. Note every place the README lied, omitted, or assumed. Those are findings against the README,
   not against the stranger.
4. Record the date on the line below.

Last run: 2026-11-25
- The README gives no `rustup target add` line for the bare-metal build; it only lists three target names, one of them in a comment. The run machine already had all three, so the builds succeeded, but a stranger would not know the step.
- The README states what it proves ("no Kani harnesses"), so it does not need Kani. It still never says that `cargo kani` is the command or how to install it. The run guessed `cargo kani`; it finished in under one second with "No proof harnesses ... were found to verify". The README gives no proof timing and no command for one harness, which is consistent only because none exist.
- The README says nothing about what a passing run prints that looks alarming. `cargo kani` prints an "unstable feature" `register_tool` warning before its summary, and the README does not explain it.
- The README names no falsification patch and no way to apply and revert one, and the repository has no `falsifications/` directory. The run substituted its own patch (changing `SYSTEM_CALL_YIELD` from `1` to `1 + 1`): `the_system_call_numbers_and_the_first_object_modules_are_pinned` failed as expected, and `git checkout src/lib.rs` restored a green run (25 unit tests, 4 doctests).
- The README says "Rust 1.88" but gives no command to check it. The run inferred `cargo +1.88 test` (25 unit tests and 4 doctests pass) and `cargo +1.87 build --lib` (refused: the package requires rustc 1.88). The README also does not say that `cargo +<version>` needs that toolchain installed through rustup.
- The README says "Nothing is machine-checked", while `.github/workflows/proofs.yml` and `docs/stranger-test.md` still speak of proofs, and `src/timer.rs` documents a Kani harness that the authors tried and dropped. A stranger reading the workflow list would expect proofs the README denies.
- Passed as documented: `cargo test` (25 unit tests, 4 doctests, about one second) and all three `--target` library builds.
