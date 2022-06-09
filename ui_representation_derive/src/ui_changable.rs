extern crate proc_macro;
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use syn::{punctuated::Punctuated, Field, Token};

use crate::{collect_fields, identify_input_types, NonStaticUIRepresentation};

pub fn impl_ui_changable(ast: &syn::DeriveInput) -> TokenStream {
    // Struct specific definitions
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let fields = collect_fields(ast);
    let changeables = get_changeables(fields);

    // The UIRepresented trait implementation
    let trait_impl = quote!(
        impl #impl_generics ::ui_representation::UIChangable for #name #ty_generics #where_clause {
          fn ui_change(&mut self, event: ::ui_representation::UIChangeEvent) -> Result<(), ::ui_representation::UIChangableError> {
            #changeables
          }
        }
    );

    trait_impl
}

fn get_changeables(fields: Vec<Field>) -> TokenStream {
    let mut changeables = Punctuated::<TokenStream, Token![,]>::new();
    let mut field_names = Punctuated::<TokenStream, Token![|]>::new();

    let representations = identify_input_types(&fields);
    representations.iter().for_each(|NonStaticUIRepresentation { field, variant }| {
      field_names.push(quote!{#field});
      let field_ident = Ident::new(&field, Span::call_site());

        let changable = match variant {
            ::ui_representation::UIRepresentationVariant::Select => quote!{
              (#field, ::ui_representation::UIChangeEvent { variant: ::ui_representation::UIChangeEventVariant::Select(value), .. }) => {
                self.#field_ident = value;
                Ok(())
              }
            },
            ::ui_representation::UIRepresentationVariant::Checkbox => quote!{
              (#field, ::ui_representation::UIChangeEvent { variant: ::ui_representation::UIChangeEventVariant::Checkbox(value), .. }) => {
                self.#field_ident = value;
                Ok(())
              }
            },
            ::ui_representation::UIRepresentationVariant::Tab(_) => quote!{
              (#field, ::ui_representation::UIChangeEvent { variant: ::ui_representation::UIChangeEventVariant::Tab(value), .. }) => {
                self.#field_ident = value;
                Ok(())
              }
            },
            ::ui_representation::UIRepresentationVariant::TextBox => quote!{
              (#field, ::ui_representation::UIChangeEvent { variant: ::ui_representation::UIChangeEventVariant::TextBox(value), .. }) => {
                self.#field_ident = value;
                Ok(())
              }
            },
            ::ui_representation::UIRepresentationVariant::TextEntry => quote!{
              (#field, ::ui_representation::UIChangeEvent { variant: ::ui_representation::UIChangeEventVariant::TextEntry(value), .. }) => {
                self.#field_ident = value;
                Ok(())
              }
            },
        };

        changeables.push(changable);
    });

    changeables.push(quote! {
      (#field_names, _) => Err(::ui_representation::UIChangableError::WrongType)
    });

    changeables.push(quote! {
      _ => Err(::ui_representation::UIChangableError::InvalidField)
    });

    let result = quote! {
      let field = event.field.clone();
      match (field.as_str(), event) {
        #changeables
      }
    };

    result
}
