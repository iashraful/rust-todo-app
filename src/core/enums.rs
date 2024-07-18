use std::fmt;

#[derive(Debug, PartialEq)]
pub enum AppMode {
    API,
    CLI,
}
impl fmt::Display for AppMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
