use crate::Level;
pub use crate::LoggingFunc;
pub use std::format_args;

pub fn log(level: Level, func: crate::LoggingFunc) {
    crate::logger().log(level, func)
}
