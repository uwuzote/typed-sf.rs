use crate::*;

// [*>[*]*<<]>
type MvChR = Cycle<Flip<Right<SetTrue<Left<Left>>>>, Right>;

#[test]
fn chunk_move() {
    eq_st!(
        Run<MvChR, State<Cons<True, Nil>, True, Nil>>,
        (Cons<False, Nil>, False, Cons<True, Cons<True, Nil>>)
    )
}
