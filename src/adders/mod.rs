pub mod half_adder;
pub mod full_adder;
pub mod adder;
pub mod incrementa;
#[derive(PartialEq, Debug)]
pub struct AdderResult {
  pub sum: bool,
  pub carry: bool,
}
