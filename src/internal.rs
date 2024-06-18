pub use crate::LoggingFunc;

pub fn log(func: LoggingFunc) {
    crate::logger().log(func)
}
