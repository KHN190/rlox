#[macro_use]
mod macros;

pub mod chunk;
mod compiler;
pub mod debug;
mod lexer;
mod run;
pub mod vm;

pub const DEBUG_LEXER: bool = false;
pub const DEBUG_TRACE: bool = true;

#[cfg(test)]
mod tests;
