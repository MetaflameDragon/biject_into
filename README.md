Generates bijective `From`/`Into` impls for two types from a single `match`-like block.

```rust
use biject::bijection;
#[derive(Debug, PartialEq, Clone)]
enum Tristate {
    Neutral,
    Positive,
    Negative,
}

bijection!(Option<bool>, Tristate, {
    None => Tristate::Neutral,
    Some(true) => Tristate::Positive,
    Some(false) => Tristate::Negative,
});

assert_eq!(Tristate::from(None::<bool>), Tristate::Neutral);
assert_eq!(Tristate::from(Some(true)), Tristate::Positive);
assert_eq!(Tristate::from(Some(false)), Tristate::Negative);

assert_eq!(Option::<bool>::from(Tristate::Neutral), None::<bool>);
assert_eq!(Option::<bool>::from(Tristate::Positive), Some(true));
assert_eq!(Option::<bool>::from(Tristate::Negative), Some(false));
```

---

This crate is currently WIP. The macro works, but it still has some issues, and parts of the API might still change. Semver compatibility is not guaranteed until (if?) published.
