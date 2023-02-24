use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

#[proc_macro_derive(Const)]
pub fn const_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    if let syn::Data::Enum(emun) = input.data {
        let consts = make_consts(&emun);
        let mod_ident = syn::Ident::new(
            &(input.ident.to_string().to_lowercase() + "_consts"),
            input.ident.span(),
        );
        let vis = input.vis;
        quote! {
            #vis mod #mod_ident {
                #consts
            }
        }
        .into()
    } else {
        quote! {
            compile_error!("Const proc macro must be used on an enum")
        }
        .into()
    }
}

fn make_consts(emun: &syn::DataEnum) -> TokenStream {
    let mut discriminant = quote! { 0 };
    let consts = emun.variants.iter().map(|variant| {
        if let Some((_, manual_discriminant)) = &variant.discriminant {
            discriminant = quote! {#manual_discriminant}
        }
        let const_ident = syn::Ident::new(
            &variant.ident.to_string().to_uppercase(),
            variant.ident.span(),
        );
        let result = quote! {pub const #const_ident: u8 = #discriminant;};
        discriminant = quote! {(#discriminant) + 1};
        result
    });
    quote! {#(#consts)*}
}

#[proc_macro_derive(AsBytes)]
pub fn as_bytes_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    if let syn::Data::Enum(emun) = input.data {
        let enum_ident = input.ident;
        let consts = make_consts(&emun);

        let match_arms = emun.variants.into_iter().map(|variant| {
            let variant_name = variant.ident;
            let fields = handle_fields(variant.fields);
            let arm_body = make_arm_body(&variant_name, &fields);
            quote! {Self::#variant_name #fields => {#arm_body}}
        });

        quote! {
            impl #enum_ident {
                fn as_bytes(&self) -> ([u8; std::mem::size_of::<#enum_ident>()], usize) {
                    #consts

                    const num_bytes: usize = std::mem::size_of::<#enum_ident>();
                    let mut result = [0; std::mem::size_of::<#enum_ident>()];
                    let bytes_used = match self {
                        #(#match_arms)*
                    };
                    (result, bytes_used)
                }
            }
        }
        .into()
    } else {
        quote! {
            compile_error!("AsBytes proc macro must be used on an enum")
        }
        .into()
    }
}

enum HandledFields {
    Named {
        fields: Vec<(syn::Ident, syn::Type)>,
    },
    Unnamed {
        fields: Vec<(syn::Ident, syn::Type)>,
    },
    Unit,
}

impl ToTokens for HandledFields {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            HandledFields::Named { fields } => {
                let mut center_tokens = TokenStream::new();
                for (ident, _) in fields {
                    ident.to_tokens(&mut center_tokens);
                    center_tokens.extend(quote! {,})
                }
                tokens.extend(quote! {
                    {#center_tokens}
                })
            }
            HandledFields::Unnamed { fields, .. } => {
                let mut center_tokens = TokenStream::new();
                for (ident, _) in fields {
                    ident.to_tokens(&mut center_tokens);
                    center_tokens.extend(quote! {,})
                }
                tokens.extend(quote! {
                    (#center_tokens)
                })
            }
            HandledFields::Unit => {}
        }
    }
}

fn handle_fields(fields: syn::Fields) -> HandledFields {
    match fields {
        syn::Fields::Named(fields) => HandledFields::Named {
            fields: fields
                .named
                .into_iter()
                .map(|field| {
                    (
                        field.ident.expect("Named field didn't have a name"),
                        field.ty,
                    )
                })
                .collect(),
        },
        syn::Fields::Unnamed(fields) => {
            let mut field_num = 0;
            HandledFields::Unnamed {
                fields: fields
                    .unnamed
                    .into_iter()
                    .map(|field| {
                        field_num += 1;
                        (
                            syn::Ident::new(&format!("field{}", field_num), field.span()),
                            field.ty,
                        )
                    })
                    .collect(),
            }
        }
        syn::Fields::Unit => HandledFields::Unit,
    }
}

fn make_arm_body(variant: &syn::Ident, fields: &HandledFields) -> TokenStream {
    let const_ident = syn::Ident::new(&variant.to_string().to_uppercase(), variant.span());
    match fields {
        HandledFields::Named { fields } | HandledFields::Unnamed { fields } => {
            let mut bytes = quote! {1};
            let set_bytes = fields.iter().map(|(name, ty)| {
                let result = quote! {
                    ::core::mem::forget(::core::mem::replace(
                        &mut <&[u8] as ::core::convert::TryInto<[u8; ::core::mem::size_of::<#ty>()]>>::try_into(
                            &result[(#bytes)..(#bytes) + ::core::mem::size_of::<#ty>()]
                        ).unwrap(),
                        unsafe {::core::mem::transmute(*#name)}
                    ));
                };
                bytes = quote! { #bytes + ::core::mem::size_of_val(&#name)};
                result
            });
            quote! {
                result[0] = #const_ident;
                #(#set_bytes)*
                #bytes
            }
        }
        HandledFields::Unit => {
            quote! {
                result[0] = #const_ident;
                1
            }
        }
    }
}
