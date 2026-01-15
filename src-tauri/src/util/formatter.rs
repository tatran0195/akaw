use std::sync::OnceLock;
use time::format_description;

static LOG_TIME_FMT: OnceLock<Vec<format_description::FormatItem<'static>>> = OnceLock::new();

pub fn log_time_fmt() -> &'static Vec<format_description::FormatItem<'static>> {
    LOG_TIME_FMT.get_or_init(|| {
        format_description::parse("[year]-[month]-[day] [hour]:[minute]:[second] UTC").unwrap()
    })
}
