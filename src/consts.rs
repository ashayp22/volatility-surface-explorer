/// Specify whether an option is put or call
#[derive(PartialEq, Debug, Copy, Clone, PartialOrd)]
pub enum OptionDir {
    CALL = 1,
    PUT = -1,
}