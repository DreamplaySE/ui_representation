use ui_representation::{UIRepresentation, UIRepresented};

#[derive(ui_representation_derive::UIRepresented)]
struct Test;

fn main() {
  let val = Test;

  let res = val.ui_representation();

  println!("{:?}", res);
}