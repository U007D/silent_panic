# silent_panic
Suppress panic messages while `SilentPanic` is in scope.

## Motivation:
Often, tests are designed to panic in order to verify behavior.  By default, Rust prints panic information to the
console.  This can be messy, making output parsing more difficult, trigger false alarms and more.  So when a test is
deliberately panicking, this crate allows the test author to silence the panic output temporarily.

`SilentPanic` will save the current (global) panic handler, replacing it with one that does not output anything, and
finally, will restore the original panic handler on `Drop`.

## License
Licensed under either:
* MIT license (see LICENSE-MIT file)
* Apache License, Version 2.0 (see LICENSE-APACHE file)
  at your option.

## Contributions
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the
work by you shall be dual licensed as above, without any additional terms or conditions.
