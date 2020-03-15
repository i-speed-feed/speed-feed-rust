use clap::{App, Arg};
use log::Level;
use std::env::*;
use speed_feed_config::YamlConfig;
use speed_feed_lib::System;
use speed_feed_ookla::OoklaAnalyzer;
use speed_feed_printer::Printer;
use speed_feed_scheduler::SchedulerPlugin;

fn main() {
    let opts = App::new("My Super Program")
        .version("0.1.0")
        .author("Felix Marezki <fmarezki@gmail.com>")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("v")
                .short("v")
                .multiple(true)
                .help("Sets the level of verbosity"),
        )
        .get_matches();

    let _ = simple_logger::init_with_level(match opts.occurrences_of("v") {
        0 => Level::Error,
        1 => Level::Warn,
        2 => Level::Info,
        3 => Level::Debug,
        4 => Level::Trace,
        _ => Level::Info,
    });

    let mut s = System::new();

    s.add(&YamlConfig {});
    s.add(&OoklaAnalyzer {});
    s.add(&Printer {});
    s.add(&SchedulerPlugin {});

    match var("SPEED_FEED_CONFIG") {
        Ok(c) => {
            s.config
                .insert(String::from("config"), c);
        },
        _ => {}
    }

    match opts.value_of("config") {
        Some(config) => {
            s.config
                .insert(String::from("config"), String::from(config));
        }
        _ => {}
    }

    loop {
        s.run();
    }
}
