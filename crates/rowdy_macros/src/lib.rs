use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, spanned::Spanned, DeriveInput};

#[proc_macro_derive(Bytes)]
pub fn bytes_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as EnumInput);
    let consts = make_consts(&input);
    let as_bytes_tokens = as_bytes(&input);
    let from_bytes_tokens = from_bytes(&input);
    quote! {
        const _: () = {
            #consts
            #as_bytes_tokens
            #from_bytes_tokens
        };
    }
    .into()
}

struct EnumInput {
    ident: syn::Ident,
    data: syn::DataEnum,
}

impl syn::parse::Parse for EnumInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let derive_input = DeriveInput::parse(input)?;
        if let syn::Data::Enum(data) = derive_input.data {
            Ok(EnumInput {
                ident: derive_input.ident,
                data,
            })
        } else {
            Err(syn::Error::new(
                input.span(),
                "Bytes proc macro must be used on an enum",
            ))
        }
    }
}

impl std::ops::Deref for EnumInput {
    type Target = syn::DataEnum;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

fn make_consts(emun: &EnumInput) -> TokenStream {
    let disc_consts = emun
        .variants
        .iter()
        .enumerate()
        .map(|(discriminant, variant)| {
            let discriminant = discriminant as u8;
            let const_ident = syn::Ident::new(
                &variant.ident.to_string().to_uppercase(),
                variant.ident.span(),
            );
            let result = quote! {const #const_ident: u8 = #discriminant;};
            result
        });

    let size_array = emun.variants.iter().map(|variant| {
        let field_sizes = variant.fields.iter().map(|field| {
            let ty = &field.ty;
            quote! {::core::mem::size_of::<#ty>()}
        });
        quote! {1 #(+ #field_sizes)*}
    });
    let mut size_array2 = size_array.clone();

    let size_consts = emun.variants.iter().map(|variant| {
        let const_ident = syn::Ident::new(
            &(variant.ident.to_string().to_uppercase() + "_SIZE"),
            variant.ident.span(),
        );
        let size = size_array2.next().unwrap();
        quote! {
            const #const_ident: usize = #size;
        }
    });

    let num_variants = &emun.variants.len();

    quote! {
        #(#disc_consts)*
        #(#size_consts)*
        const SIZES: [usize; #num_variants] = [#(#size_array),*];
    }
}

// TODO: endianness
fn as_bytes(input: &EnumInput) -> TokenStream {
    let enum_ident = &input.ident;

    let match_arms = input.variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let fields = handle_fields(&variant.fields);
        let arm_body = make_arm_body(&variant_name, &fields);
        quote! {Self::#variant_name #fields => {#arm_body}}
    });

    quote! {
        impl #enum_ident {
            fn as_bytes(&self) -> ([u8; std::mem::size_of::<#enum_ident>()], usize) {
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

fn handle_fields(fields: &syn::Fields) -> HandledFields {
    match fields {
        syn::Fields::Named(fields) => HandledFields::Named {
            fields: fields
                .named
                .iter()
                .map(|field| {
                    (
                        field.ident.clone().expect("Named field didn't have a name"),
                        field.ty.clone(),
                    )
                })
                .collect(),
        },
        syn::Fields::Unnamed(fields) => {
            let mut field_num = 0;
            HandledFields::Unnamed {
                fields: fields
                    .unnamed
                    .iter()
                    .map(|field| {
                        field_num += 1;
                        (
                            syn::Ident::new(&format!("field{}", field_num), field.span()),
                            field.ty.clone(),
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
                    ::core::mem::forget(unsafe{::core::mem::replace(
                        (&mut result[(#bytes)..(#bytes + ::core::mem::size_of::<#ty>())]
                            as *mut _ as *mut [u8; ::core::mem::size_of::<#ty>()]
                        ).as_mut()
                        .unwrap(),
                        ::core::mem::transmute(*#name)
                    )});
                };
                bytes = quote! { #bytes + ::core::mem::size_of::<#ty>()};
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

fn from_bytes(input: &EnumInput) -> TokenStream {
    let enum_ident = &input.ident;

    let match_arms = input
        .variants
        .iter()
        .enumerate()
        .map(|(discriminant, variant)| {
            let discriminant = discriminant as u8;
            let handled_fields = handle_fields(&variant.fields);
            let variant_name = &variant.ident;

            // *(&bytes[1..5] as *const [u8] as *const i32)

            match handled_fields {
                HandledFields::Named { fields } => {
                    let mut byte = quote! {1};
                    let parsed_fields = fields.into_iter().map(|(name, ty)| {
                        let result = quote! { #name: *(&bytes[(#byte)..(#byte + ::core::mem::size_of::<#ty>())] as *const _ as *const #ty) };
                        byte = quote! {#byte + ::core::mem::size_of::<#ty>()};
                        result
                    });
                    quote! {
                        #discriminant => unsafe { Self::#variant_name {#(#parsed_fields),*} },
                    }
                }
                HandledFields::Unnamed { fields } => {
                    let mut byte = quote! {1};
                    let parsed_fields = fields.into_iter().map(|(_, ty)| {
                        let result = quote! { *(&bytes[(#byte)..(#byte) + ::core::mem::size_of::<#ty>()] as *const [u8] as *const #ty) };
                        byte = quote!{(#byte) + ::core::mem::size_of::<#ty>()};
                        result
                    });
                    quote! {
                        #discriminant => unsafe { Self::#variant_name(#(#parsed_fields),*) },
                    }
                }
                HandledFields::Unit => quote! {#discriminant => Self::#variant_name,},
            }
        });

    quote! {
        impl #enum_ident {
            fn from_bytes(bytes: &[u8]) -> (Self, usize) {
                let kind = bytes[0];
                let size = SIZES[kind as usize];
                (
                    match kind {
                        #(#match_arms)*
                        _ => unsafe{::core::hint::unreachable_unchecked()},
                    },
                    size
                )
            }
        }
    }
    .into()
}
