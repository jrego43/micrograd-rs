#![crate_name = "micrograd_rs"]

mod ops;

pub mod optimizer;
pub use optimizer as optim;

mod criterion;
pub use criterion::*;

mod activation;
pub use activation::*;

mod modules;
pub use modules::*;

pub mod prelude;
pub mod utils;

mod tensor;
pub use tensor::Tensor;

mod value;
pub use value::Value;
