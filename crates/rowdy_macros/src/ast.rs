use quote::quote;
use std::collections::BTreeMap;
use syn::{parse_macro_input, spanned::Spanned, Token};

pub fn ast(
    attrs: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let modd = parse_macro_input!(input as syn::ItemMod);

    let mods = parse_macro_input!(attrs with syn::punctuated::Punctuated::<syn::Ident, Token![,]>::parse_terminated);
    for ident in mods.iter() {
        if ident == &modd.ident.to_string() {
            return syn::Error::new_spanned(
                ident,
                "cannot create a mod of the same name as the original",
            )
            .to_compile_error()
            .into();
        }
    }

    let attrs = modd.attrs;
    let vis = modd.vis;
    let base_ident = modd.ident;
    let content = if let Some((_, content)) = modd.content {
        content
    } else {
        return syn::Error::new_spanned(
            modd.mod_token,
            "annotate macro can only be applied to a mod",
        )
        .to_compile_error()
        .into();
    };

    let mut new_mods: BTreeMap<_, _> = mods
        .iter()
        .map(|ident| {
            (
                ident.to_string(),
                syn::ItemMod {
                    attrs: attrs.clone(),
                    vis: vis.clone(),
                    mod_token: modd.mod_token,
                    ident: ident.clone(),
                    content: Some((syn::token::Brace::default(), Vec::new())),
                    semi: modd.semi,
                },
            )
        })
        .chain([(
            base_ident.to_string(),
            syn::ItemMod {
                attrs: attrs.clone(),
                vis: vis.clone(),
                mod_token: modd.mod_token,
                ident: base_ident.clone(),
                content: Some((syn::token::Brace::default(), Vec::new())),
                semi: modd.semi,
            },
        )])
        .collect();

    let mut errors = Vec::new();

    for mut item in content {
        if let syn::Item::Struct(mut item_struct) = item {
            let attrs = extract_ast_attrs(&mut item_struct.attrs);
            let struct_annotation = match StructAnnotation::new(attrs) {
                Ok(a) => a,
                Err(errs) => {
                    errors.extend(errs);
                    continue;
                }
            };

            if struct_annotation.annotations.is_empty() {
                new_mods.values_mut().for_each(|modd| {
                    add_content(modd, item_struct.clone());
                });
            } else {
                add_content(
                    new_mods.get_mut(&base_ident.to_string()).unwrap(),
                    item_struct.clone(),
                );
                for annotation in struct_annotation.annotations {
                    match new_mods.get_mut(&annotation.module.to_string()) {
                        Some(modd) => {
                            let additional_fields = annotation.fields.named;
                            let syn::ItemStruct {
                                ref attrs,
                                ref vis,
                                ref struct_token,
                                ref ident,
                                ref generics,
                                ..
                            } = item_struct;
                            let new_struct: syn::ItemStruct = syn::parse_quote! {
                                #(#attrs)*
                                #vis #struct_token #ident #generics {
                                    inner: super::#base_ident::#ident #generics,
                                    #additional_fields
                                }
                            };
                            add_content(modd, new_struct);

                            let new_deref: syn::ItemImpl = syn::parse_quote! {
                                impl #generics ::std::ops::Deref for #ident #generics {
                                    type Target = super::#base_ident::#ident #generics;

                                    fn deref(&self) -> &Self::Target {
                                        &self.inner
                                    }
                                }
                            };
                            add_content(modd, new_deref);

                            let new_deref_mut: syn::ItemImpl = syn::parse_quote! {
                                impl #generics ::std::ops::DerefMut for #ident #generics {
                                    fn deref_mut(&mut self) -> &mut Self::Target {
                                        &mut self.inner
                                    }
                                }
                            };
                            add_content(modd, new_deref_mut);
                        }
                        None => errors.push(syn::Error::new_spanned(
                            annotation.module,
                            "module not specified in original macro invocation or the base module",
                        )),
                    }
                }
            }
        } else if let syn::Item::Enum(mut item_enum) = item {
            let attrs = extract_ast_attrs(&mut item_enum.attrs);
            let struct_annotation = match StructAnnotation::new(attrs) {
                Ok(a) => a,
                Err(errs) => {
                    errors.extend(errs);
                    continue;
                }
            };

            if struct_annotation.annotations.is_empty() {
                new_mods.values_mut().for_each(|modd| {
                    add_content(modd, item_enum.clone());
                });
            } else {
                add_content(
                    new_mods.get_mut(&base_ident.to_string()).unwrap(),
                    item_enum.clone(),
                );
                for annotation in struct_annotation.annotations {
                    match new_mods.get_mut(&annotation.module.to_string()) {
                        Some(modd) => {
                            let additional_fields = annotation.fields.named;
                            let syn::ItemEnum {
                                ref attrs,
                                ref vis,
                                ref ident,
                                ref generics,
                                ..
                            } = item_enum;
                            let new_struct: syn::ItemStruct = syn::parse_quote! {
                                #(#attrs)*
                                #vis struct #ident #generics {
                                    inner: super::#base_ident::#ident #generics,
                                    #additional_fields
                                }
                            };
                            add_content(modd, new_struct);

                            let new_deref: syn::ItemImpl = syn::parse_quote! {
                                impl #generics ::std::ops::Deref for #ident #generics {
                                    type Target = super::#base_ident::#ident #generics;

                                    fn deref(&self) -> &Self::Target {
                                        &self.inner
                                    }
                                }
                            };
                            add_content(modd, new_deref);

                            let new_deref_mut: syn::ItemImpl = syn::parse_quote! {
                                impl #generics ::std::ops::DerefMut for #ident #generics {
                                    fn deref_mut(&mut self) -> &mut Self::Target {
                                        &mut self.inner
                                    }
                                }
                            };
                            add_content(modd, new_deref_mut);
                        }
                        None => errors.push(syn::Error::new_spanned(
                            annotation.module,
                            "module not specified in original macro invocation or the base module",
                        )),
                    }
                }
            }
        } else {
            let attrs = extract_ast_attrs(match get_attrs(&mut item) {
                Some(attrs) => attrs,
                None => {
                    new_mods.values_mut().for_each(|modd| {
                        add_content(modd, item.clone());
                    });
                    continue;
                }
            });

            let item_annotations = match ItemAnnotations::new(attrs) {
                Ok(a) => a,
                Err(errs) => {
                    errors.extend(errs);
                    continue;
                }
            };

            match item_annotations {
                ItemAnnotations::All => new_mods.values_mut().for_each(|modd| {
                    add_content(modd, item.clone());
                }),
                ItemAnnotations::Some(mods) => {
                    for mod_name in mods {
                        match new_mods.get_mut(&mod_name.to_string()) {
                            Some(modd) => {
                                add_content(modd, item.clone());
                            },
                            None => errors.push(syn::Error::new_spanned(
                                mod_name,
                                "module not specified in original macro invocation or the base module",
                            )),
                        }
                    }
                }
            }
        }
    }

    if errors.is_empty() {
        let new_mods = new_mods.values();
        quote! {
            #(#new_mods)*
        }
        .into()
    } else {
        let errors = errors.into_iter().map(|err| err.into_compile_error());
        quote! {
            #(#errors)*
        }
        .into()
    }
}

struct Annotation {
    module: syn::Ident,
    fields: syn::FieldsNamed,
}

fn extract_ast_attrs(attrs: &mut Vec<syn::Attribute>) -> Vec<syn::Attribute> {
    let result;
    (result, *attrs) = attrs.drain(..).partition(|attr| {
        let segments = &attr.path.segments;
        if let Some(first) = segments.first() {
            first.ident == "ast" && first.arguments.is_none()
        } else {
            false
        }
    });
    // AstAnnotations::from_attrs(result.into_iter())
    result
}

fn add_content(modd: &mut syn::ItemMod, new_content: impl Into<syn::Item>) -> Option<()> {
    modd.content.as_mut()?.1.push(new_content.into());
    Some(())
}

fn get_attrs(item: &mut syn::Item) -> Option<&mut Vec<syn::Attribute>> {
    match item {
        syn::Item::Const(item) => Some(&mut item.attrs),
        syn::Item::Enum(item) => Some(&mut item.attrs),
        syn::Item::ExternCrate(item) => Some(&mut item.attrs),
        syn::Item::Fn(item) => Some(&mut item.attrs),
        syn::Item::ForeignMod(item) => Some(&mut item.attrs),
        syn::Item::Impl(item) => Some(&mut item.attrs),
        syn::Item::Macro(item) => Some(&mut item.attrs),
        syn::Item::Macro2(item) => Some(&mut item.attrs),
        syn::Item::Mod(item) => Some(&mut item.attrs),
        syn::Item::Static(item) => Some(&mut item.attrs),
        syn::Item::Struct(item) => Some(&mut item.attrs),
        syn::Item::Trait(item) => Some(&mut item.attrs),
        syn::Item::TraitAlias(item) => Some(&mut item.attrs),
        syn::Item::Type(item) => Some(&mut item.attrs),
        syn::Item::Union(item) => Some(&mut item.attrs),
        syn::Item::Use(item) => Some(&mut item.attrs),
        _ => None,
    }
}

struct StructAnnotation {
    annotations: Vec<Annotation>,
}

impl StructAnnotation {
    fn new(attrs: Vec<syn::Attribute>) -> Result<Self, Vec<syn::Error>> {
        let mut errors = Vec::new();
        let mut annotations = Vec::new();
        for attr in attrs {
            let mut segments = attr.path.segments.iter();

            match segments.next() {
                Some(first) => {
                    if first.ident != "ast" || !first.arguments.is_none() {
                        errors.push(syn::Error::new_spanned(
                            attr,
                            "First part of attribute was not `ast`. This error should never occur",
                        ));
                        continue;
                    }
                }
                None => {
                    errors.push(syn::Error::new_spanned(
                        attr,
                        "First part of attribute was not `ast`. This error should never occur",
                    ));
                    continue;
                }
            }

            match segments.next() {
                Some(mod_name) => {
                    if !mod_name.arguments.is_none() {
                        errors.push(syn::Error::new(
                            mod_name.arguments.span(),
                            "module name cannot have arguments",
                        ));
                    }
                    match segments.next() {
                        Some(other) => {
                            errors
                                .push(syn::Error::new_spanned(other, "Max 2 parts of ast marker"));
                        }
                        None => {}
                    }
                    // annotation
                    match syn::parse2::<syn::FieldsNamed>(attr.tokens) {
                        Ok(fields) => annotations.push(Annotation {
                            fields,
                            module: mod_name.ident.clone(),
                        }),
                        Err(error) => errors.push(error),
                    }
                }
                None => {
                    errors.push(syn::Error::new_spanned(
                        attr,
                        "Expected a module name after ast",
                    ));
                }
            }
        }
        if errors.is_empty() {
            Ok(Self { annotations })
        } else {
            Err(errors)
        }
    }
}

enum ItemAnnotations {
    All,
    Some(Vec<syn::Ident>),
}

impl ItemAnnotations {
    fn new(attrs: Vec<syn::Attribute>) -> Result<Self, Vec<syn::Error>> {
        if attrs.is_empty() {
            return Ok(Self::All);
        }

        let mut errors = Vec::new();
        let mut modules = Vec::new();
        for attr in attrs {
            let mut segments = attr.path.segments.iter();

            match segments.next() {
                Some(first) => {
                    if first.ident != "ast" || !first.arguments.is_none() {
                        errors.push(syn::Error::new_spanned(
                            attr,
                            "First part of attribute was not `ast`. This error should never occur",
                        ));
                        continue;
                    }
                }
                None => {
                    errors.push(syn::Error::new_spanned(
                        attr,
                        "First part of attribute was not `ast`. This error should never occur",
                    ));
                    continue;
                }
            }

            match segments.next() {
                Some(mod_name) => {
                    if !mod_name.arguments.is_none() {
                        errors.push(syn::Error::new(
                            mod_name.arguments.span(),
                            "module name cannot have arguments",
                        ));
                    }
                    match segments.next() {
                        Some(other) => {
                            errors
                                .push(syn::Error::new_spanned(other, "Max 2 parts of ast marker"));
                        }
                        None => {}
                    }
                    if attr.tokens.is_empty() {
                        modules.push(mod_name.ident.clone())
                    } else {
                        errors.push(syn::Error::new_spanned(
                            attr.tokens,
                            "Only structs can have additional information here",
                        ))
                    }
                }
                None => {}
            }
        }
        if errors.is_empty() {
            if modules.is_empty() {
                Ok(Self::All)
            } else {
                Ok(Self::Some(modules))
            }
        } else {
            Err(errors)
        }
    }
}
