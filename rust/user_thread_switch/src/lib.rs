#![feature(naked_functions)]

const DEFAULT_STACK_SIZE: usize = 1024 * 8 * 4;
const MAX_TASKS: usize = 1000 * 1000;

mod gettime;
mod runtime;

pub use gettime::*;
pub use runtime::*;
