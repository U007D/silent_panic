#![allow(clippy::unwrap_used)]

#[allow(unused_imports)]
use super::*;
use assert2::assert;
use crossbeam_utils::thread::scope;

#[test]
fn normal_panic_emits_noise() {
    // Given
    let panicking_thread = scope(|scope| scope.spawn(|_| panic!()));

    // When
    let result = todo!();

    // Then
    assert!(result == expected);
}
