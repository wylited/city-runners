use chrono::Local;

use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

type LogGuard = WorkerGuard; // Type alias

pub fn init() -> LogGuard {
    let now = Local::now();
    // Log file naming convention, cr-year-month-date_hour:minute:second.log
    let log_file = format!("cr-{}.log", now.format("%Y-%m-%d_%H:%M:%S").to_string());

    // create a non blocking writer
    // and its guard so that when the application is exited the log file is not corrupted
    let (writer, guard) =
        tracing_appender::non_blocking(tracing_appender::rolling::never(".log", &log_file));
    // Never rolling means that it never continues log files from previous runtimes.

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_writer(writer))
        .init(); // initialize the tracing with the file writer.

    guard // return the guard so that the application owns it
}

// To kill the WorkerGuard, do drop.
