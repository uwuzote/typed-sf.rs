use crate::bit::Bit;
use core::marker::PhantomData;

/// Type-level linked list, enum-like of [`Nil`] and [`Cons`]
pub trait List {
    #[cfg(not(feature = "no_std"))]
    /// Real value of type.
    fn val() -> Vec<bool>;
}

/// End of [`List`].
#[derive(Debug)]
pub struct Nil;

/// Node of list, consists of value ([`Bit`]) and tail -- [`List`].
#[derive(Debug)]
pub struct Cons<Value, Tail>(PhantomData<(Value, Tail)>)
where
    Value: Bit,
    Tail: List;

impl List for Nil {
    #[cfg(not(feature = "no_std"))]
    fn val() -> Vec<bool> {
        Vec::new()
    }
}

impl<Value, Tail> List for Cons<Value, Tail>
where
    Value: Bit,
    Tail: List,
{
    #[cfg(not(feature = "no_std"))]
    fn val() -> Vec<bool> {
        let mut vec = Tail::val();
        vec.push(Value::val());

        vec
    }
}
