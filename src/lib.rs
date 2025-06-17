macro_rules! bijection {
    // Final construction of the From impls
    (@
    ($first_ty:ty, $second_ty:ty)
        { $($first_done:tt)* }
        { $($second_done:tt)* }
        ()
        ()
    ) => {
        // Extra block so this can be used without a semicolon (expands to an expression)
        {
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
            ()
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
        )
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
        )
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
        )
    };
}

#[cfg(test)]
mod tests {
    use std::fmt::Debug;

    use super::*;

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
}
