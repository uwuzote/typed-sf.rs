#![allow(warnings)]

use crate::*;

#[macro_export]
macro_rules! eq_st {
    ($a:ty, ($bl:ty, $bv:ty, $br:ty)) => {
        assert_eq!(
            <$a as StateTrait>::val(),
            <State<$bl, $bv, $br, False> as StateTrait>::val()
        );
    };
    ($a:ty, $b:ty) => {
        assert_eq!(<$a as StateTrait>::val(), <$b as StateTrait>::val());
    };
}

mod complex;
mod flip;
mod left;
mod utils;
