extern crate proc_macro;
use proc_macro::TokenStream;

#[derive(Debug, Clone, Copy)]
pub enum UIRepresentation {
  Select,
  Checkbox,
}

pub trait UIRepresented {
    fn ui_representation(&self) -> &'static UIRepresentation;
}