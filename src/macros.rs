#[macro_export]
macro_rules! log {
    ($lvl:expr, $fmt:expr, $($arg:tt)+) => {
        // let tid = unsafe{libc::gettid() as u32};
        let func = $crate::internal::LoggingFunc::new(
            move |rolling_logger: &mut $crate::RollingLogger, file: &str, line: u32, tid:u32, lvl:&str| {
                let date_time = chrono::Local::now();
                let _ = rolling_logger.rollate_with_datetime(&date_time);
                let time_str = date_time.format("%H:%M:%S%.9f").to_string();
                let ss = format!($fmt, $($arg)+);
                ufmt::uwriteln!(rolling_logger, "{} {} {}:{} {} {}", time_str.as_str(), tid, file, line, lvl, ss.as_str()).unwrap();
            },
            std::file!(),
            std::line!(),
            111 as u32,
            $lvl
        );
        $crate::internal::log($lvl, func);
    };

    ($lvl:expr, $fmt:expr) => {
        // let tid = unsafe{libc::gettid() as u32};

        let func = $crate::internal::LoggingFunc::new(
            move |rolling_logger: &mut $crate::RollingLogger, file: &str, line: u32, tid:u32, lvl: &str| {
                let date_time = chrono::Local::now();
                let _ = rolling_logger.rollate_with_datetime(&date_time);
                let time_str = date_time.format("%H:%M:%S%.9f").to_string();
                ufmt::uwriteln!(rolling_logger, "{} {} {}:{} {} {}", time_str.as_str(), tid, file, line, lvl, $fmt).unwrap();
            },
            std::file!(),
            std::line!(),
            111 as u32,
            $lvl
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
