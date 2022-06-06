extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{quote};

#[proc_macro_derive(UIRepresented, attributes(input))]
pub fn derive_ui_represent(_item: TokenStream) -> TokenStream {
  let ast = syn::parse(_item).unwrap();
  impl_ui_represent(&ast).into()
}

fn impl_ui_represent(ast: &syn::DeriveInput) -> proc_macro2::TokenStream {
    // Struct specific definitions
    let name = &ast.ident;
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    // The UIRepresented trait implementation
    let trait_impl = quote!(
        impl #impl_generics ui_representation::UIRepresented for #name #ty_generics #where_clause {
          fn ui_representation(&self) -> &'static UIRepresentation { todo!() }
        }
    );

    println!("{}", trait_impl.to_string());

    trait_impl
}