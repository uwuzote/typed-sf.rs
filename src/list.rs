use crate::bit::Bit;

/// Type-level linked list, enum-like of [`Nil`] and [`Cons`]
pub trait List {
    #[cfg(not(feature = "no_std"))]
    /// `Real`, represented with rust's objects value of type.
    fn val() -> Vec<bool>;

    /// `Real` value of type.
    fn new() -> Self;
}

/// End of [`List`].
#[derive(Debug)]
pub struct Nil;

/// Node of list, consists of value ([`Bit`]) and tail -- [`List`].
#[derive(Debug)]
pub struct Cons<Value, Tail>(Value, Tail)
where
    Value: Bit,
    Tail: List;

impl List for Nil {
    #[cfg(not(feature = "no_std"))]
    #[inline]
    fn val() -> Vec<bool> {
        Vec::new()
    }

    #[inline]
    fn new() -> Self {
        Nil
    }
}

impl<Value, Tail> List for Cons<Value, Tail>
where
    Value: Bit,
    Tail: List,
{
    #[cfg(not(feature = "no_std"))]
    #[inline]
    fn val() -> Vec<bool> {
        let mut vec = Tail::val();
        vec.push(Value::val());

        vec
    }

    #[inline]
    fn new() -> Self {
        Cons(Value::new(), Tail::new())
    }
}
