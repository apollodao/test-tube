use crate::runner::Runner;

pub mod bank;
pub mod wasm;

#[macro_use]
pub mod macros;

pub trait Module<'a, R: Runner<'a>> {
    fn new(runner: &'a R) -> Self;
}
