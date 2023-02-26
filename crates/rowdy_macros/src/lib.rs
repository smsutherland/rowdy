mod bytes;

#[proc_macro_derive(Bytes)]
pub fn bytes_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    bytes::bytes_derive(input)
}
