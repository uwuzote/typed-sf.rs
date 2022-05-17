#![allow(warnings)]

#[macro_export]
macro_rules! eq_st {
    ($a:ty, ($bl:ty, $bv:ty, $br:ty)) => {
        assert_eq!(
            <$a as StateTrait>::val(),
            <State<$bl, $bv, $br> as StateTrait>::val()
        );
    };
    ($a:ty, $b:ty) => {
        assert_eq!(<$a as _State>::val(), <$b as _State>::val());
    };
}

pub(super) mod prelude {
    // #[allow(unused_imports)]
    pub(crate) use crate::{
        eq_st,
        Cons,
        Cycle,
        DefaultState,
        False,
        Flip,
        Left,
        Nil,
        Right,
        Run,
        State,
        // traits
        StateTrait,
        True,
        EOF,
    };
}

mod flip;
mod left;
