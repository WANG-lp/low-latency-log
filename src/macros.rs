#[macro_export]
macro_rules! log {
    ($lvl:expr, $fmt:expr, $($arg:tt)+) => {
        // let file = std::file!();
        // let line = std::line!();
        let func = $crate::internal::LoggingFunc::new(
            move |rolling_logger: &mut $crate::RollingLogger| {
                // let _ = rolling_logger.write_with_datetime(file.as_bytes());
                // let _ = rolling_logger.write_with_datetime(line);

                // ufmt::uwriteln!(&mut rolling_logger, $fmt, $($arg)+).unwrap();
            },
            std::file!(),
            std::line!(),
        );
        $crate::internal::log($lvl, func);
    };

    ($lvl:expr, $fmt:expr) => {
        // let time_point = chrono::Local::now();
        // let tid = thread_id::get();
        // let file = std::file!();
        // let line = std::line!();
        let func = $crate::internal::LoggingFunc::new(
            move |rolling_logger: &mut $crate::RollingLogger| {
                // let _ = rolling_logger.write_with_datetime(file.as_bytes());
                // ufmt::uwriteln!(&mut rolling_logger, $fmt).unwrap();
            },
            std::file!(),
            std::line!(),
        );
        $crate::internal::log($lvl, func);
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => ($crate::log!($crate::LogLevel::Error, $($arg)+));
    ($fmt:expr) => ($crate::log!($crate::Level::Error, expr))
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => ($crate::log!($crate::LogLevel::Warn, $($arg)+));
    ($fmt:expr) => ($crate::log!($crate::Level::Warn, expr))

}

#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => ($crate::log!($crate::LogLevel::Info, $($arg)+));
    ($fmt:tt) => ($crate::log!($crate::Level::Info, expr))
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => ($crate::log!($crate::LogLevel::Debug, $($arg)+));
    ($fmt:expr) => ($crate::log!($crate::Level::Debug, expr))
}
