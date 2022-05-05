// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
    Debug,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Error => error(&message),
        LogLevel::Info => info(&message),
        LogLevel::Warning => warn(&message),
        LogLevel::Debug => debug(&message),
    }
}

pub fn info(message: &str) -> String {
    let mut info: String = "[INFO]: ".to_string();
    info.push_str(message);
    info
}
pub fn warn(message: &str) -> String {
    let mut warn: String = "[WARNING]: ".to_string();
    warn.push_str(message);
    warn
}
pub fn error(message: &str) -> String {
    let mut error: String = "[ERROR]: ".to_string();
    error.push_str(message);
    error
}
pub fn debug(message: &str) -> String {
    let mut debug: String = "[DEBUG]: ".to_string();
    debug.push_str(message);
    debug
}
