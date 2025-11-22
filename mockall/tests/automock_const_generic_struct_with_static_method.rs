// vim: tw=80
//! static non-generic methods of generic structs shouldn't require any special
//! treatment when mocking.
#![deny(warnings)]

use mockall::*;
use std::sync::{Mutex, PoisonError};

static A_MTX: Mutex<()> = Mutex::new(());

pub struct A<const L: usize, const C: bool> {}

#[automock]
impl<const L: usize, const C: bool> A<L, C> {
    pub fn bar() -> [u8; L] {unimplemented!()}
}

#[test]
fn generic_return() {
    let _m = A_MTX.lock().unwrap_or_else(PoisonError::into_inner);

    let ctx = MockA::<1_usize, true>::bar_context();
    ctx.expect().return_const([13_u8]);
    assert_eq!([13_u8], MockA::<1_usize, true>::bar());
}

#[should_panic]
#[test]
fn wrong_const_generic_expectation() {
    let _m = A_MTX.lock().unwrap_or_else(PoisonError::into_inner);

    let ctx = MockA::<1, true>::bar_context();
    ctx.expect().return_const([13]);

    // We expect a panic, because the context above is for a different const generic value.
    MockA::<1, false>::bar();
}
