use crate::*;

#[allow(non_camel_case_types)]
type p = Left<EOF>;

#[test]
fn left_cons() {
    eq_st!(
        Run<p, State<Cons<False, Nil>, True, Nil, False>>,
        (Nil, False, Cons<True, Nil>)
    );

    eq_st!(
        Run<p, State<Cons<True, Nil>, False, Nil, False>>,
        (Nil, True, Cons<False, Nil>)
    );
}

#[test]
fn left_nil_false() {
    type p = Left<EOF>;

    eq_st!(
        Run<p, DefaultState>,
        (Nil, False, Cons<False, Nil>)
    );
}

#[test]
fn left_nil_true() {
    type p = Left<EOF>;

    eq_st!(
        Run<p, DefaultState<True>>,
        (Nil, True, Cons<True, Nil>)
    );
}
