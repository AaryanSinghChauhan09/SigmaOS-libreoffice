//! Options model.

#[derive(Debug, Clone, PartialEq)]
pub enum Mode { Default, Writer(String), Calc(String), Impress(String), Draw(String), Save(String) }

#[derive(Debug, Clone)]
pub struct Options { pub mode: Mode }
impl Default for Options { fn default() -> Self { Self { mode: Mode::Default } } }
#[cfg(test)] mod tests { use super::*; #[test] fn o() { assert!(matches!(Options::default().mode, Mode::Default)); } }
