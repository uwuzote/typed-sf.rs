/// Type-level enum of [`True`] and [`False`].
pub trait Bit {
    /// Result of negation of bit
    type Neg: Bit;

    /// `Real`, represented with rust's objects value of type.
    fn val() -> bool;

    /// `Real` value of type.
    fn new() -> Self;
}

/// Type-level True value, [`Bit`].
#[derive(Debug)]
pub struct True;

/// Type-level False value, [`Bit`].
#[derive(Debug)]
pub struct False;

// /// Default [`Bit`] for filling gaps ([`False`]).
// #[allow(clippy::module_name_repetitions)]
// pub type FillBit = False;

impl Bit for True {
    type Neg = False;

    #[inline(always)]
    fn val() -> bool {
        true
    }

    #[inline(always)]
    fn new() -> Self {
        True
    }
}

impl Bit for False {
    type Neg = True;

    #[inline(always)]
    fn val() -> bool {
        false
    }

    #[inline(always)]
    fn new() -> Self {
        False
    }
}
