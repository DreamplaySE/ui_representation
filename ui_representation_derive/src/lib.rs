use proc_macro_error::{abort, proc_macro_error};
use quote::ToTokens;
use syn::{parse_quote, spanned::Spanned, Field};
use ui_changable::impl_ui_changable;
use ui_presentation::impl_ui_represent;
use ui_representation::UIRepresentationVariant;

mod ui_changable;
mod ui_presentation;

extern crate proc_macro;

#[proc_macro_derive(UIRepresented, attributes(ui_represented))]
#[proc_macro_error]
pub fn derive_ui_represented(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(_item).unwrap();
    impl_ui_represent(&ast).into()
}

#[proc_macro_derive(UIChangable, attributes(ui_represented))]
#[proc_macro_error]
pub fn derive_ui_changable(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(_item).unwrap();
    impl_ui_changable(&ast).into()
}

fn collect_fields(ast: &syn::DeriveInput) -> Vec<syn::Field> {
    match ast.data {
        syn::Data::Struct(syn::DataStruct { ref fields, .. }) => {
            if fields.iter().any(|field| field.ident.is_none()) {
                abort!(
                    fields.span(),
                    "struct has unnamed fields";
                    help = "#[derive(UIRepresented)] can only be used on structs with named fields";
                );
            }
            fields.iter().cloned().collect::<Vec<_>>()
        }
        _ => abort!(
            ast.span(),
            "#[derive(UIRepresented)] can only be used with structs"
        ),
    }
}

struct NonStaticUIRepresentation {
    pub field: String,
    pub variant: UIRepresentationVariant,
}

fn identify_input_types(fields: &Vec<syn::Field>) -> Vec<NonStaticUIRepresentation> {
    fields
        .iter()
        .filter_map(
            |Field {
                 ident, attrs, ty, ..
             }| {
                let field_ident = ident.clone().unwrap();
                let field = field_ident.to_string();

                let mut variant: Option<UIRepresentationVariant> = None;

                for attr in attrs {
                    if attr.path != parse_quote!(ui_represented) {
                        continue;
                    }

                    match attr.parse_meta() {
                        Ok(syn::Meta::List(syn::MetaList { ref nested, .. })) => {
                            match nested.first() {
                                Some(syn::NestedMeta::Meta(input_type)) => {
                                    match input_type {
                                        syn::Meta::Path(path) => {
                                            //Specified UI Implementation
                                            let ident = path.get_ident().unwrap();

                                            match ident.to_string().as_ref() {
                                                "checkbox" => {
                                                    variant =
                                                        Some(UIRepresentationVariant::Checkbox);
                                                }
                                                _ => abort!(
                                                    attr.span(),
                                                    "The provided type does not exist"
                                                ),
                                            }
                                        }
                                        _ => abort!(attr.span(), "Only provide one value"),
                                    }
                                }
                                _ => abort!(attr.span(), "No type given"),
                            }
                        }
                        Ok(syn::Meta::Path(_)) => {
                            //Default UI representation
                            match ty {
                                syn::Type::Path(ref ty) => {
                                    let ident = &ty.path;
                                    match ident.to_token_stream().to_string().as_ref() {
                                        "bool" => {
                                            variant = Some(UIRepresentationVariant::Checkbox);
                                        }
                                        "String" => {
                                            variant = Some(UIRepresentationVariant::TextEntry);
                                        }
                                        dat => abort!(
                                            attr.span(),
                                            format!(
                                                r#"No default implementation for type {:?}."#,
                                                dat
                                            )
                                        ),
                                    }
                                }
                                _ => abort!(
                                    attr.span(),
                                    "No default implementation for unknown type."
                                ),
                            }
                        }
                        Ok(syn::Meta::NameValue(_)) => {
                            abort!(attr.span(), "Unexpected name=value argument")
                        }
                        Err(_) => todo!(),
                    };
                }

                if let Some(variant) = variant {
                    Some(NonStaticUIRepresentation { field, variant })
                } else {
                    None
                }
            },
        )
        .collect()
}
