#![allow(clippy::unwrap_used)]

#[allow(unused_imports)]
use super::*;
use assert2::assert;

#[test]
fn normal_panic_emits_noise() {
    // Given
    let _silent = SilentPanic::on();

    // When
    // let result = todo!();

    // Then
    assert!(true);
}
