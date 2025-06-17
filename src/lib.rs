#![feature(trace_macros)]

macro_rules! bijection {
    ($first_ty:ty, $second_ty:ty,
        {$($bij:tt)*}
    ) => {
        bijection!(@
            ($first_ty, $second_ty)
            {}
            {}
            ($($bij)*)
            ($($bij)*)
        )
    };

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

    (@
    ($first_ty:ty, $second_ty:ty)
        { $($first_done:tt)* }
        { $($second_done:tt)* }
        ($first_pat:pat_param => $first_expr:expr      $(,)?)
        ($second_expr:expr    => $second_pat:pat_param $(,)?)
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_variants() {
        enum Foo {
            A,
            B,
        }

        enum Bar {
            X,
            Y,
        }

        trace_macros!(true);
        bijection!(Foo, Bar, {
            Foo::A => Bar::X,
            Foo::B => Bar::Y,
        });
        trace_macros!(false);
    }
}
