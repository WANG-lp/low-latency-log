pub const LEVEL_FILTER: crate::LogLevel = LEVEL_FILTER_INNER;

#[macro_export]
macro_rules! log {
    ($lvl:expr, $fmt:expr, $($arg:tt)+) => {
        use std::fmt::Write;
        if $lvl >= $crate::macros::LEVEL_FILTER{
            let tid = $crate::TID.get();
            let system_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64;
            let func = $crate::internal::LoggingFunc::new(
                move || {
                    let mut s = String::with_capacity(255);
                    write!(&mut s, $fmt, $($arg)+).unwrap();
                    s.into()
                },
                std::file!(),
                std::line!(),
                tid,
                $lvl,
                system_time,
            );
            $crate::internal::log(func);
        }
    };

    ($lvl:expr, $fmt:expr) => {
        if $lvl >= $crate::macros::LEVEL_FILTER{
            let tid = $crate::TID.get();
            let system_time = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u64;
            let func = $crate::internal::LoggingFunc::new(
                move || {
                $fmt.into()
                },
                std::file!(),
                std::line!(),
                tid,
                $lvl,
                system_time
            );
            $crate::internal::log(func);
        }
    };
}

use cfg_if::cfg_if;

use crate::LogLevel;

cfg_if! {
  if #[cfg(feature = "level-off")] {
        const LEVEL_FILTER_INNER: LogLevel = LogLevel::Off;
    } else if #[cfg(feature = "level-error")] {
        const LEVEL_FILTER_INNER: LogLevel = LogLevel::Error;
    } else if #[cfg(feature = "level-warn")] {
        const LEVEL_FILTER_INNER: LogLevel = LogLevel::Warn;
    } else if #[cfg(feature = "level-info")] {
        const LEVEL_FILTER_INNER: LogLevel = LogLevel::Info;
    } else if #[cfg(feature = "level-debug")] {
        const LEVEL_FILTER_INNER: LogLevel = LogLevel::Debug;
    } else {
        const LEVEL_FILTER_INNER: LogLevel = LogLevel::Trace;
    }
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

#[macro_export]
macro_rules! trace {
    ($($arg:tt)+) => ($crate::log!($crate::LogLevel::Trace, $($arg)+));
    ($fmt:expr) => ($crate::log!($crate::Level::Trace, expr))
}
