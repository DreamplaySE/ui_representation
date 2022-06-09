use std::{error::Error, fmt};

use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize)]
pub enum UIRepresentationVariant {
  Select,
  Checkbox,
  TextBox,
  TextEntry,
  Tab(&'static [UIRepresentation]),
}

#[derive(Debug, Clone, Serialize)]
pub struct UIRepresentation {
  pub field: &'static str,
  pub variant: UIRepresentationVariant,
}

pub trait UIRepresented {
  fn ui_representation() -> &'static [UIRepresentation];

  fn own_ui_representation(&self) -> &'static [UIRepresentation] {
      Self::ui_representation()
  }
}

#[derive(Debug)]
pub enum UIChangableError {
  WrongType,
  InvalidField,
}

impl Error for UIChangableError {}

impl fmt::Display for UIChangableError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      match self {
        UIChangableError::WrongType => write!(f, "The type of the field is not correct."),
        UIChangableError::InvalidField => write!(f, "The type specified does not exist"),
    }
  }
}

pub trait UIChangable {
    fn ui_change(&mut self, event: UIChangeEvent) -> Result<(), UIChangableError>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIChangeEvent {
  pub field: String,
  pub variant: UIChangeEventVariant,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UIChangeEventVariant {
  Select,
  Checkbox(bool),
  TextBox(String),
  TextEntry(String),
  Tab(Box<UIChangeEvent>),
}