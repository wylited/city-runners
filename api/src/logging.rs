use std::{
    fs::{self, File},
    path::PathBuf,
    sync::Arc,
};

use chrono::Local;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init() {
    let log_dir = "./logs";
    fs::create_dir_all(log_dir).expect("Failed to create log directory");
    let mut log_file = PathBuf::from("./logs");

    // Log file naming convention, cr-year-month-date_hour:minute:second.log
    log_file.push(format!(
        "cr-{}.log",
        Local::now().format("%Y-%m-%d_%H:%M:%S").to_string()
    ));

    // Create the file
    let file = File::create(log_file);
    let file = match file {
        Ok(file) => file,
        Err(error) => panic!("Error: {:?}", error),
    };

    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer().with_writer(Arc::new(file)))
        .with(tracing_subscriber::fmt::layer().with_target(true).pretty())
        .init(); // initialize the tracing with the file writer.
}
