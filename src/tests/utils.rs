use crate::*;

#[test]
fn set_true() {
    eq_st!(
        Run<SetTrue, State<Nil, False, Nil, False>>,
        (Nil, True, Nil)
    );

    eq_st!(
        Run<SetTrue, State<Nil, True, Nil, False>>,
        (Nil, True, Nil)
    );
}

#[test]
fn set_false() {
    eq_st!(
        Run<SetFalse, State<Nil, False, Nil, False>>,
        (Nil, False, Nil)
    );

    eq_st!(
        Run<SetFalse, State<Nil, True, Nil, False>>,
        (Nil, False, Nil)
    );
}
