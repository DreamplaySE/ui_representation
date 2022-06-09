extern crate proc_macro;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{punctuated::Punctuated, Field, Token};

use crate::{collect_fields, identify_input_types, NonStaticUIRepresentation};

pub fn impl_ui_represent(ast: &syn::DeriveInput) -> TokenStream {
    // Struct specific definitions
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let fields = collect_fields(ast);
    let representations = get_representations(fields);

    // The UIRepresented trait implementation
    let trait_impl = quote!(
        impl #impl_generics ::ui_representation::UIRepresented for #name #ty_generics #where_clause {
          fn ui_representation() -> &'static [::ui_representation::UIRepresentation] {
            &#representations
          }
        }
    );

    trait_impl
}

fn get_representations(fields: Vec<Field>) -> TokenStream {
    let mut representations = Punctuated::<TokenStream, Token![,]>::new();

    let representation_variables = identify_input_types(&fields);

    representation_variables
        .iter()
        .for_each(|NonStaticUIRepresentation { field, variant }| {
            let changable = match variant {
                ::ui_representation::UIRepresentationVariant::Select => quote! {
                  ::ui_representation::UIRepresentation {
                    field: #field,
                    variant: ::ui_representation::UIRepresentationVariant::Select,
                  }
                },
                ::ui_representation::UIRepresentationVariant::Checkbox => quote! {
                  ::ui_representation::UIRepresentation {
                    field: #field,
                    variant: ::ui_representation::UIRepresentationVariant::Checkbox,
                  }
                },
                ::ui_representation::UIRepresentationVariant::Tab(_) => todo!(),
                ::ui_representation::UIRepresentationVariant::TextBox => quote! {
                  ::ui_representation::UIRepresentation {
                    field: #field,
                    variant: ::ui_representation::UIRepresentationVariant::TextBox,
                  }
                },
                ::ui_representation::UIRepresentationVariant::TextEntry => quote! {
                  ::ui_representation::UIRepresentation {
                    field: #field,
                    variant: ::ui_representation::UIRepresentationVariant::TextEntry,
                  }
                },
            };

            representations.push(changable);
        });

    let result = quote! {
      [
        #representations
      ]
    };

    result
}
