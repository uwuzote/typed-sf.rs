use crate::*;

#[test]
fn flip_false() {
    eq_st!(Run<Flip<EOF>, DefaultState>, (Nil, True, Nil));
}

#[test]
fn flip_true() {
    eq_st!(Run<Flip<EOF>, DefaultState<True>>, (Nil, False, Nil));
}
