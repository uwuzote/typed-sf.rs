/// Type-level enum of [`True`] and [`False`].
pub trait Bit {
    /// Result of negation of bit
    type Neg: Bit;

    /// Real value of type.
    fn val() -> bool;
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

    fn val() -> bool {
        true
    }
}

impl Bit for False {
    type Neg = True;

    fn val() -> bool {
        false
    }
}
