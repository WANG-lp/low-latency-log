use std::borrow::Cow;

#[derive(Debug, Default)]
pub struct LogProxy {}

impl log::Log for LogProxy {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        let lvl: crate::LogLevel = metadata.level().into();
        lvl >= crate::macros::LEVEL_FILTER
    }

    fn log(&self, record: &log::Record) {
        let lvl: crate::LogLevel = record.metadata().level().into();
        let tid = crate::TID.get();
        let system_time = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        // TODO: opt here
        let args = match record.args().as_str() {
            Some(s) => Cow::Borrowed(s),
            None => Cow::Owned(record.args().to_string()),
        };
        let func = crate::internal::LoggingFunc::new(
            move || args.clone(),
            record.file_static().unwrap_or(""),
            record.line().unwrap_or(0) as u32,
            tid,
            lvl,
            system_time,
        );
        crate::internal::log(func);
    }
    fn flush(&self) {
        crate::Logger::flush();
    }
}
