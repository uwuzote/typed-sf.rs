Simple, ready-to-use typed version of [SmallF*ck](https://esolangs.org/wiki/Smallfuck) esoteric language. Highly inspired by [this article](https://sdleffler.github.io/RustTypeSystemTuringComplete/).

```rust
use typed_sf::*;
type SetTrue<Next = EOF> = Cycle<Flip, Flip<Next>>;
// [*<<[*]*>>>] // Move any-sized chunk of True's 2 cells left

#[allow(non_camel_case_types)]
type prog = Cycle<Flip<Left<Left<SetTrue<Right<Right<Right>>>>>>>;

#[allow(non_camel_case_types)]
type result = Run<
    prog,
    State<Nil, True, Cons<True, Nil>>
>;

assert_eq!(
    <result as StateTrait>::val(),
    (vec![true, true, false, false], false, Vec::new())
);
```