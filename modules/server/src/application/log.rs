//! Logging module
use fastrace::collector::{Config, ConsoleReporter};
use std::io::Write;

use fastrace::prelude::{Event, SpanContext};

/// Setup logging, TODO (#16) make this configurable.
pub(crate) fn setup_logger() {
    fastrace::set_reporter(ConsoleReporter, Config::default());
    env_logger::Builder::from_default_env()
        .format(|buf, record| {
            // Convert every log to an event in the current local parent span
            Event::add_to_local_parent(record.level().as_str(), || {
                [("message".into(), record.args().to_string().into())]
            });

            // Attach the current trace id to the log message
            if let Some(current) = SpanContext::current_local_parent() {
                writeln!(
                    buf,
                    "[{}]: {} | {}",
                    record.level(),
                    current.trace_id.0,
                    record.args()
                )
            } else {
                writeln!(buf, "[{}]: {}", record.level(), record.args())
            }
        })
        .filter_level(log::LevelFilter::Debug)
        .init();
}
