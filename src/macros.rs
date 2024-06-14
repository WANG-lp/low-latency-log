#[macro_export]
macro_rules! log {
    ($lvl:expr, $fmt:expr, $($arg:tt)+) => {
        let func = $crate::internal::LoggingFunc::new(move || {
            // fomat_macros::fomat!($($arg)+)
            let mut s = $crate::UString { 0: String::new() };
            ufmt::uwriteln!(&mut s, $fmt, $($arg)+).unwrap();
            s.0
        });
        $crate::internal::log($lvl, func);
    };

    ($lvl:expr, $fmt:expr) => {
        let func = $crate::internal::LoggingFunc::new(move || {
            // fomat_macros::fomat!($fmt)
            let mut s = $crate::UString { 0: String::new() };
            ufmt::uwriteln!(&mut s, $fmt).unwrap();
            s.0
        });
        $crate::internal::log($lvl, func);
    };
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)+) => ($crate::log!($crate::Level::Error, $($arg)+));
    ($fmt:expr) => ($crate::log!($crate::Level::Error, expr))
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)+) => ($crate::log!($crate::Level::Warn, $($arg)+));
    ($fmt:expr) => ($crate::log!($crate::Level::Warn, expr))

}

#[macro_export]
macro_rules! info {
    ($($arg:tt)+) => ($crate::log!($crate::Level::Info, $($arg)+));
    ($fmt:tt) => ($crate::log!($crate::Level::Info, expr))
}

#[macro_export]
macro_rules! debug {
    ($($arg:tt)+) => ($crate::log!($crate::Level::Debug, $($arg)+));
    ($fmt:expr) => ($crate::log!($crate::Level::Debug, expr))
}
