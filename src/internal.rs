use crate::LogLevel;
pub use crate::LoggingFunc;
pub use std::format_args;

pub fn log(level: LogLevel, func: LoggingFunc) {
    crate::logger().log(level, func)
}
