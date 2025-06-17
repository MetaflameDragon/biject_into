/// Generates `From` impls for any two types, providing conversions between them.
/// This is effectively a shorthand for creating two duplicate `match` statements
/// with the sides swapped.
///
/// This is especially useful for quickly mapping between two enums,
/// but this works for any two types and any patterns.
///
/// # Usage
/// ```text
/// bijection!(Foo, Bar, {
///     Foo::A => Bar::X,
///     Foo::B(b) => Bar::Y(b),
///     Foo::C { x } => Bar::Z { x },
///     // ...
/// });
/// ```
/// The bijection expressions are very similar to `match` branches.
/// Because of the two-way nature of bijection, both sides must be valid patterns (without alternates)
/// and expressions - that is, both `Foo::A => Bar::X` and `Bar::X => Foo::A` must be valid in a match expression.
///
/// # Examples
/// ```rust
/// use biject::bijection;
///
/// #[derive(Debug, PartialEq, Clone)]
/// struct Point {
///     x: i32,
///     y: i32,
/// }
///
/// #[derive(Debug, PartialEq, Clone)]
/// enum PointEnum {
///     Zero,
///     OneOne,
///     Other { x: i32, y: i32 },
/// }
///
/// bijection!(Point, PointEnum, {
///             Point { x: 0, y: 0 } => PointEnum::Zero,
///             Point { x: 1, y: 1 } => PointEnum::OneOne,
///             Point { x, y } => PointEnum::Other { x, y },
///         });
///
/// assert_eq!(PointEnum::from(Point { x: 0, y: 0 }), PointEnum::Zero);
/// assert_eq!(PointEnum::from(Point { x: 1, y: 1 }), PointEnum::OneOne);
/// assert_eq!(PointEnum::from(Point { x: 5, y: 10 }), PointEnum::Other { x: 5, y: 10 });
/// assert_eq!(PointEnum::from(Point { x: 20, y: -20 }), PointEnum::Other { x: 20, y: -20 });
///
/// assert_eq!(Point::from(PointEnum::Zero), Point { x: 0, y: 0 });
/// assert_eq!(Point::from(PointEnum::OneOne), Point { x: 1, y: 1 });
/// assert_eq!(Point::from(PointEnum::Other { x: 5, y: 10 }), Point { x: 5, y: 10 });
/// assert_eq!(Point::from(PointEnum::Other { x: 20, y: -20 }), Point { x: 20, y: -20 });
/// ```
///
/// ```rust
/// use biject::bijection;
/// #[derive(Debug, PartialEq, Clone)]
/// enum Tristate {
///     Neutral,
///     Positive,
///     Negative,
/// }
///
/// bijection!(Option<bool>, Tristate, {
///             None => Tristate::Neutral,
///             Some(true) => Tristate::Positive,
///             Some(false) => Tristate::Negative,
///         });
///
/// assert_eq!(Tristate::from(None::<bool>), Tristate::Neutral);
/// assert_eq!(Tristate::from(Some(true)), Tristate::Positive);
/// assert_eq!(Tristate::from(Some(false)), Tristate::Negative);
///
/// assert_eq!(Option::<bool>::from(Tristate::Neutral), None::<bool>);
/// assert_eq!(Option::<bool>::from(Tristate::Positive), Some(true));
/// assert_eq!(Option::<bool>::from(Tristate::Negative), Some(false));
/// ```
///
/// # Caveats
///
/// ## Unreachable patterns
/// By itself, the macro does not enforce bijection
/// ```rust
/// # use biject::bijection;
/// # #[derive(Debug, PartialEq, Clone)]
/// # struct Foo(i32);
///
/// # #[derive(Debug, PartialEq, Clone)]
/// # struct Bar(i32);
///
///
/// bijection!(Foo, Bar, {
///     Foo(0) => Bar(0),
///     Foo(1) => Bar(0), // Bar(0) is unreachable!
///     Foo(1) => Bar(1), // Foo(1) is unreachable!
///     Foo(x) => Bar(x),
/// });
/// ```
/// Luckily, this will still cause the macro to emit `unreachable_patterns` warnings,
/// and even highlight the problematic patterns for you.
///
/// You may wrap the macro in a block (or a module) and annotate it with `#[deny(unreachable_patterns)]`.
///
/// ## Bijection branches
/// The bijection branches are structured to look like `match` branches, but unlike the latter,
/// or-patterns (or any ambiguous patterns) are disallowed.
/// ```rust,compile_fail
/// # use biject::bijection;
/// # #[derive(Debug, PartialEq, Clone)]
/// # struct Foo(i32);
///
/// # #[derive(Debug, PartialEq, Clone)]
/// # struct Bar(i32);
///
/// bijection!(Foo, Bar, {
///     // No or-pattern! That would be like writing Bar(0) => Foo(0) | Foo(1)
///     Foo(0) | Foo(1) => Bar(0),
///     // Inner or-patterns currently pass compilation,
///     // and unexpectedly produce a bitwise or instead!
///     Foo(2 | 3) => Bar(1),
///     Foo(x) => Bar(x),
/// });
/// ```
/// Using an inner or-pattern may currently compile successfully, but it (incorrectly) produces
/// a bitwise or instead.
#[macro_export]
macro_rules! bijection {
    // Final construction of the From impls
    (@
    ($first_ty:ty, $second_ty:ty)
        { $($first_done:tt)* }
        { $($second_done:tt)* }
        ()
        ()
    ) => {
        impl From<$first_ty> for $second_ty {
            fn from(value: $first_ty) -> Self {
                match value {
                    $($first_done)*
                }
            }
        }

        impl From<$second_ty> for $first_ty {
            fn from(value: $second_ty) -> Self {
                match value {
                    $($second_done)*
                }
            }
        }
    };

    // Entry
    ($first_ty:ty, $second_ty:ty,
        {$($bij:tt)*}
    ) => {
        bijection!(@
            ($first_ty, $second_ty)
            {}
            {}
            // Double up the bijection statements for matching
            ($($bij)*)
            ($($bij)*)
        );
    };

    // Normalize by munching rules sequentially
    // This matches the initial $($bij)* with two macro patterns at once
    (@
    ($first_ty:ty, $second_ty:ty)
        { $($first_done:tt)* }
        { $($second_done:tt)* }
        ($first_pat:pat_param => $first_expr:expr      , $($first_rest:tt )*)
        ($second_expr:expr    => $second_pat:pat_param , $($second_rest:tt)*)
    ) => {
        bijection!(@
            ($first_ty, $second_ty)
            {
                $($first_done)*
                $first_pat => $first_expr,
            }
            {
                $($second_done)*
                $second_pat => $second_expr,
            }
            ($($first_rest)*)
            ($($second_rest)*)
        );
    };

    // Normalization without the trailing comma
    (@
    ($first_ty:ty, $second_ty:ty)
        { $($first_done:tt)* }
        { $($second_done:tt)* }
        ($first_pat:pat_param => $first_expr:expr     )
        ($second_expr:expr    => $second_pat:pat_param)
    ) => {
        bijection!(@
            ($first_ty, $second_ty)
            {
                $($first_done)*
                $first_pat => $first_expr,
            }
            {
                $($second_done)*
                $second_pat => $second_expr,
            }
            ()
            ()
        );
    };

    // ===== Invalid patterns for better compiler errors =====

    // Notes:
    // - Using the matched type tokens somehow helps with highlighting (at least in RustRover).
    //   This is done via loose `let` declarations.
    // - Similarly for other tokens

    // Internal macro errors

    // Invalid bijection match statements (e.g. Foo::A = Bar::X)
    (@
    ($first_ty:ty, $second_ty:ty)
        { $($first_done:tt)* }
        { $($second_done:tt)* }
        ($($first_rest:tt )*)
        ($($second_rest:tt)*)
    ) => {
        {
            let _: $first_ty;
            let _: $second_ty;
            // This match statement might produce a better (native) compiler error message
            // Example: `Foo::A = Bar::X` will make it complain about needing `=>` instead
            // The #allow suppresses an unnecessary lint
            #[allow(unreachable_code)]
            match unreachable!() {
                $($first_rest)*
            };
            compile_error!(concat!("Invalid bijection pattern:\n", stringify!($($first_rest)*)));
        }
    };

    // Fallback
    (@ $($unknown:tt)*) => {
        {
            // Uncomment stringify and comment out the compiler error for debugging
            // const _: &str = concat!($(stringify!($unknown)),*);
            compile_error!("Uncaught internal macro error");
        }
    };

    // Incorrect syntax

    // Ex: bijection!(Foo, Bar)
    ($first_ty:ty, $second_ty:ty $(,)?) => {
        {
            let _: $first_ty;
            let _: $second_ty;
            compile_error!("Missing bijection declaration block after types");
        }
    };

    // Ex: bijection!(Foo, Bar {})
    ($first_ty:ty, $second_ty:tt $bij:block) => {
        {
            let _: $first_ty;
            let _: $second_ty;
            compile_error!("Bijection declaration block must be separated with a comma");
        }
    };

    // Ex: bijection!(Foo, Bar, Foo::A => Bar::X)
    ($first_ty:ty, $second_ty:tt, $($bij:tt)+) => {
        {
            let _: $first_ty;
            let _: $second_ty;
            compile_error!(
                concat!(
                    "Bijection declaration block expected (got: ",
                    stringify!($($bij)+),
                    ")"
                )
            );
        }
    };

    // Same as the above without the comma
    // Ex: bijection!(Foo, Bar Foo::A => Bar::X)
    ($first_ty:ty, $second_ty:tt $($bij:tt)+) => {
        {
            let _: $first_ty;
            let _: $second_ty;
            compile_error!(
                concat!(
                    "Bijection declaration block expected (got: ",
                    stringify!($($bij)+),
                    ")"
                )
            );
        }
    };

    // Ex: bijection!(Foo, { Foo::A => Bar::X })
    ($first_ty:ty $(, $($bij:tt)*)?) => {
        {
            let _: $first_ty;
            compile_error!("Missing second type");
        }
    };

    // Ex: bijection!(Foo { Foo::A => Bar::X })
    // Note: Slightly unhelpful compiler error message (will complain about `=>` in blocks)
    ($first_ty:ty $bij:block) => {
        {
            let _: $first_ty;
            compile_error!("Bijection declaration block must be separated with a comma");
        }
    };

    // Ex: bijection!({ Foo::A => Bar::X })
    // Catches anything contained in curly braces without the leading types
    ({$($bij:tt)*}) => {
        compile_error!("Missing types before declaration block");
    };

    // Fallback, catches everything else
    ($($unknown:tt)*) => {
        compile_error!("Expected: TypeA, TypeB, { /* bijection patterns */ }");
    };
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

    mod context_usage_tests {
        mod context_mod {
            enum Foo {
                A,
                B,
            }
            enum Bar {
                X,
                Y,
            }

            // Can be used within modules
            bijection!(Foo, Bar, {
                Foo::A => Bar::X,
                Foo::B => Bar::Y,
            });
        }

        mod context_fn {
            #[allow(dead_code)]
            fn foo() {
                enum Foo {
                    A,
                    B,
                }
                enum Bar {
                    X,
                    Y,
                }

                // Can be used within functions
                bijection!(Foo, Bar, {
                    Foo::A => Bar::X,
                    Foo::B => Bar::Y,
                });
            }
        }
    }

    /// Asserts equality in both directions,
    /// and that `T` and `U` both implement `From`/`Into` for one another.
    fn test_bijection_eq<T, U>(t: T, u: U)
    where
        T: From<U> + PartialEq + Debug + Clone,
        U: From<T> + PartialEq + Debug + Clone,
    {
        assert_eq!(U::from(t.clone()), u.clone());
        assert_eq!(T::from(u), t);
    }

    #[test]
    fn empty_enum() {
        #[derive(Debug, PartialEq, Clone)]
        enum Foo {}

        #[derive(Debug, PartialEq, Clone)]
        enum Bar {}

        bijection!(Foo, Bar, {});
    }

    #[test]
    fn one_variant() {
        #[derive(Debug, PartialEq, Clone)]
        enum Foo {
            A,
        }

        #[derive(Debug, PartialEq, Clone)]
        enum Bar {
            X,
        }

        bijection!(Foo, Bar, {
            Foo::A => Bar::X,
        });

        test_bijection_eq(Foo::A, Bar::X);
    }

    #[test]
    fn two_variants() {
        #[derive(Debug, PartialEq, Clone)]
        enum Foo {
            A,
            B,
        }

        #[derive(Debug, PartialEq, Clone)]
        enum Bar {
            X,
            Y,
        }

        bijection!(Foo, Bar, {
            Foo::A => Bar::X,
            Foo::B => Bar::Y,
        });

        test_bijection_eq(Foo::A, Bar::X);
        test_bijection_eq(Foo::B, Bar::Y);
    }

    #[test]
    fn struct_point() {
        #[derive(Debug, PartialEq, Clone)]
        struct Point {
            x: i32,
            y: i32,
        }

        #[derive(Debug, PartialEq, Clone)]
        struct PointFlipped {
            y: i32,
            x: i32,
        }

        bijection!(Point, PointFlipped, {
            Point { x, y } => PointFlipped { y: x, x: y }
        });

        test_bijection_eq(Point { x: 5, y: 10 }, PointFlipped { x: 10, y: 5 });
        test_bijection_eq(Point { x: 20, y: -20 }, PointFlipped { x: -20, y: 20 });
    }

    #[test]
    fn struct_enum() {
        #[derive(Debug, PartialEq, Clone)]
        struct Point {
            x: i32,
            y: i32,
        }

        #[derive(Debug, PartialEq, Clone)]
        enum PointEnum {
            Zero,
            OneOne,
            Other { x: i32, y: i32 },
        }

        bijection!(Point, PointEnum, {
            Point { x: 0, y: 0 } => PointEnum::Zero,
            Point { x: 1, y: 1 } => PointEnum::OneOne,
            Point { x, y } => PointEnum::Other { x, y },
        });

        test_bijection_eq(Point { x: 0, y: 0 }, PointEnum::Zero);
        test_bijection_eq(Point { x: 1, y: 1 }, PointEnum::OneOne);
        test_bijection_eq(Point { x: 5, y: 10 }, PointEnum::Other { x: 5, y: 10 });
        test_bijection_eq(Point { x: 20, y: -20 }, PointEnum::Other { x: 20, y: -20 });
    }

    #[test]
    fn unreachable_patterns() {
        #[derive(Debug, PartialEq, Clone)]
        struct Foo(i32);

        #[derive(Debug, PartialEq, Clone)]
        struct Bar(i32);

        // This should complain with a warning! (hence the deny attribute)
        #[deny(unfulfilled_lint_expectations)]
        #[expect(unreachable_patterns)]
        {
            bijection!(Foo, Bar, {
                Foo(0) => Bar(0),
                Foo(1) => Bar(0),
                Foo(1) => Bar(1),
                Foo(x) => Bar(x),
            });
        }

        assert_eq!(Bar::from(Foo(0)), Bar(0));
        assert_eq!(Foo::from(Bar(0)), Foo(0));

        assert_eq!(Bar::from(Foo(1)), Bar(0));
        assert_ne!(Foo::from(Bar(1)), Foo(0));
        assert_eq!(Foo::from(Bar(1)), Foo(1));

        assert_eq!(Bar::from(Foo(3)), Bar(3));
        assert_eq!(Foo::from(Bar(3)), Foo(3));
    }

    #[test]
    fn external_type() {
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

        test_bijection_eq(Tristate::Neutral, None::<bool>);
        test_bijection_eq(Tristate::Positive, Some(true));
        test_bijection_eq(Tristate::Negative, Some(false));
    }

    // TODO: Make it fail on inner or-patterns (which makes it behave like a bitwise or!!!)
    // Example Foo(2 | 3) => Bar(1),

    // TODO: Compiler error tests

    // // Used for testing compiler errors etc.
    // #[test]
    // fn playground() {
    //     #[derive(Debug, PartialEq, Clone)]
    //     enum Tristate {
    //         Neutral,
    //         Positive,
    //         Negative,
    //     }
    //
    //     bijection!(Option<bool>, Tristate, {
    //         None <=> Tristate::Neutral,
    //         Some(true) => Tristate::Positive,
    //         Some(false) => Tristate::Negative,
    //     });
    // }
}
