use log::{error, info, warn};
use speed_feed_lib::*;
use std::option::*;
use std::process::Command;

pub struct OoklaAnalyzer {}

impl OoklaAnalyzer {
    const CLI: &'static str = "speedtest";
}

impl Plugin for OoklaAnalyzer {
    fn name(&self) -> &'static str {
        "Speed :: Feed :: Ookla"
    }

    fn prio(&self) -> u32 {
        return 1;
    }

    fn run(&self, s: &mut System) {
        let cli = match s.config.get("ookla.cli") {
            Some(cli) => cli,
            _ => {
                warn!("No value for ookla.cli specified. Falling back to default.");
                OoklaAnalyzer::CLI
            }
        };

        let mut command = Command::new(cli);

        command.arg("--format=json").arg("--accept-license");

        info!("Running Ookla: {:?}", command);

        let output = match command.output() {
            Err(e) => {
                warn!("Error while running Ookla: {}", e);
                String::from("")
            }
            Ok(output) => String::from_utf8(output.stdout).expect("Ookla output i not UTF-8"),
        };

        let json = ajson::parse(output.as_str()).expect("Ookla output cannot be parsed to json");

        s.metrics.ping = json
            .get("ping.latency")
            .expect("Ookla analyzer has no ping value")
            .to_u64();

        s.metrics.down = json
            .get("download.bandwidth")
            .expect("Ookla analyzer has no download value")
            .to_u64();

        s.metrics.up = json
            .get("upload.bandwidth")
            .expect("Ookla analyzer has no upload value")
            .to_u64();
    }
}

#[cfg(test)]
mod ookla_tests {
    use crate::OoklaAnalyzer;
    use log::Level;
    use simple_logger;
    use speed_feed_lib::System;

    #[test]
    fn test_ookla_plugin() {
        let _ = simple_logger::init_with_level(Level::Debug);

        let mut s = System::new();

        s.add(&OoklaAnalyzer {});

        if cfg!(feature = "ci") {
            s.config.insert(String::from("ookla.cli"), String::from("ookla/speedtest"));
        }

        s.run();

        assert_ne!(s.metrics.ping, 0);
        assert_ne!(s.metrics.up, 0);
        assert_ne!(s.metrics.down, 0);
    }
}
