pub mod typed;
pub mod untyped;

use rowdy_location::Span;

pub trait Spanned {
    fn span(&self) -> Span;
}

#[macro_export]
macro_rules! Token {
    [,] => {$crate::Comma};
    [;] => {$crate::End};
    [+] => {$crate::Plus};
    [+=] => {$crate::PlusAssign};
    [++] => {$crate::Increment};
    [-] => {$crate::Sub};
    [-=] => {$crate::SubAssign};
    [--] => {$crate::Decrement};
    [=] => {$crate::Assign};
    [==] => {$crate::Equals};
}

macro_rules! make_node {
    ($name:ident) => {
        #[derive(Debug)]
        pub struct $name {
            pub span: Span,
        }

        impl Spanned for $name {
            fn span(&self) -> Span {
                self.span
            }
        }
    };
}

make_node! {LParen}
make_node! {RParen}
make_node! {LBrace}
make_node! {RBrace}
make_node! {LBracket}
make_node! {RBracket}
make_node! {Comma}
make_node! {End}

make_node! {Plus}
make_node! {PlusAssign}
make_node! {Increment}
make_node! {Sub}
make_node! {SubAssign}
make_node! {Decrement}
make_node! {Assign}
make_node! {Equals}

make_node! {If}
make_node! {Else}
make_node! {While}
make_node! {For}
make_node! {Return}
