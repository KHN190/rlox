#[macro_use]
mod macros;

mod chunk;
mod run;

pub mod debug;
pub mod vm;

pub const DEBUG_TRACE: bool = true;

#[cfg(test)]
mod tests;
