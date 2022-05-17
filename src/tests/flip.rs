use super::prelude::*;

#[test]
fn flip_false() {
    eq_st!(Run<Flip<EOF>, State<Nil, False, Nil>>, (Nil, True, Nil));
}

#[test]
fn flip_true() {
    eq_st!(Run<Flip<EOF>, State<Nil, True, Nil>>, (Nil, False, Nil));
}
