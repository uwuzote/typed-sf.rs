use super::prelude::*;

#[allow(non_camel_case_types)]
type p = Left<EOF>;

#[test]
fn left_cons() {
    eq_st!(
        Run<p, State<Cons<False, Nil>, True, Nil>>,
        (Nil, False, Cons<True, Nil>)
    );

    eq_st!(
        Run<p, State<Cons<True, Nil>, False, Nil>>,
        (Nil, True, Cons<False, Nil>)
    );
}

#[test]
fn left_nil_false() {
    type p = Left<EOF, False>;

    eq_st!(
        Run<p, State<Nil, False, Nil>>,
        (Nil, False, Cons<False, Nil>)
    );

    eq_st!(
        Run<p, State<Nil, True, Nil>>,
        (Nil, False, Cons<True, Nil>)
    );
}

#[test]
fn left_nil_true() {
    type p = Left<EOF, True>;

    eq_st!(
        Run<p, State<Nil, False, Nil>>,
        (Nil, True, Cons<False, Nil>)
    );

    eq_st!(
        Run<p, State<Nil, True, Nil>>,
        (Nil, True, Cons<True, Nil>)
    );
}
