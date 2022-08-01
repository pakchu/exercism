// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => return info(message),
        LogLevel::Warning => return warn(message),
        LogLevel::Error => return error(message),
        // _ => return String::from(""),
    }
}
pub fn info(message: &str) -> String {
    let mut res = String::from("[INFO]: ");
    res.push_str(message);
    res
}
pub fn warn(message: &str) -> String {
    let mut res = String::from("[WARNING]: ");
    res.push_str(message);
    res
}
pub fn error(message: &str) -> String {
    let mut res = String::from("[ERROR]: ");
    res.push_str(message);
    res
}
