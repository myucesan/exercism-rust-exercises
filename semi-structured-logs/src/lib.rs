// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

/// various log levels
#[derive(Clone, PartialEq, Debug)]
pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    let mut emission = String::from("");
    match level {
        LogLevel::Info => {
            emission.push_str("[INFO]: ");
            emission.push_str(message);

            emission
        },
        LogLevel::Warning => {
            emission.push_str("[WARNING]: ");
            emission.push_str(message);

            emission
        },
        LogLevel::Error => {            
        emission.push_str("[ERROR]: ");
        emission.push_str(message);

        emission
        }
}
}
pub fn info(message: &str) -> String {
    log(LogLevel::Info, message)
}
pub fn warn(message: &str) -> String {
    log(LogLevel::Warning, message)
}
pub fn error(message: &str) -> String {
    log(LogLevel::Error, message)
}
