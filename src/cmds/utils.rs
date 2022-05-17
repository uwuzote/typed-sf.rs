use crate::{Cycle, Flip, EOF};

/// Sets current bit to [`False`][crate::False].
pub type SetFalse<Next = EOF> = Cycle<Flip, Next>;

/// Sets current bit to [`True`][crate::True].
pub type SetTrue<Next = EOF> = SetFalse<Flip<Next>>;
