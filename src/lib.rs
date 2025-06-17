macro_rules! bijection {
    ($first_ty:ty, $second_ty:ty,
        $($left:tt)*
    ) => {
        // Extra block so this can be used without a semicolon (expands to an expression)
        {
            impl From<$first_ty> for $second_ty {
                fn from(value: $first_ty) -> Self {
                    match value {
                        // $($left => { todo!() },)+
                        _ => todo!(),
                    }
                }
            }

            impl From<$second_ty> for $first_ty {
                fn from(value: $second_ty) -> Self {
                    match value {
                        //$($right => { todo!() },)+
                        _ => todo!(),
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

        bijection!(Foo, Bar, {
            Foo::A = Bar::X,
            Foo::B = Bar::Y
        })
    }
}
