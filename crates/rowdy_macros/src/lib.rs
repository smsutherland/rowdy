mod bytes;
mod ast;

#[proc_macro_derive(Bytes)]
pub fn bytes_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    bytes::bytes_derive(input)
}

#[proc_macro_attribute]
pub fn ast(attrs: proc_macro::TokenStream, input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    ast::ast(attrs, input)
}


#[test]
fn a() {
    let _: syn::FieldsNamed = syn::parse_str("{typ: ::rowdy_types::FnSignature}").unwrap();
}