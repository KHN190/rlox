#[macro_use]
mod macros;

mod chunk;
pub mod debug;
mod run;
mod scan;
pub mod vm;

pub const DEBUG_TRACE: bool = true;

#[cfg(test)]
mod tests;
