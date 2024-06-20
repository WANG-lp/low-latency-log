use std::borrow::Cow;

#[derive(Debug, Default)]
pub struct LogProxy {}

impl log::Log for LogProxy {
    fn enabled(&self, _metadata: &log::Metadata) -> bool {
        true
    }

    fn log(&self, record: &log::Record) {
        let lvl: crate::LogLevel = record.metadata().level().into();
        if lvl >= crate::macros::LEVEL_FILTER {
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
                move || args.to_owned(),
                std::file!(),
                std::line!(),
                tid,
                lvl,
                system_time,
            );
            crate::internal::log(func);
        }
    }
    fn flush(&self) {}
}
