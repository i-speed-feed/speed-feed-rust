use speed_feed_lib::*;
use std::process::Command;

pub struct OoklaAnalyzer {}

impl OoklaAnalyzer {
    const CLI: &'static str = "speedtest";
}

impl Plugin for OoklaAnalyzer {
    fn name(&self) -> &'static str {
        "Ookla Analyzer"
    }

    fn prio(&self) -> u32 {
        return 1;
    }

    fn run(&self, s: &mut System) {
        let output = Command::new(OoklaAnalyzer::CLI)
            .arg("--format=json")
            .output()
            .expect("Ookla analyzer failed to execute");

        let json = ajson::parse(
            String::from_utf8(output.stdout).expect("Ookla output i not UTF-8").as_str()
        ).expect("Ookla output cannot be parsed to json");

        s.metrics.ping = json.get("ping.latency")
            .expect("Ookla analyzer has no ping value")
            .to_u64();

        s.metrics.down = json.get("download.bandwidth")
            .expect("Ookla analyzer has no download value")
            .to_u64();

        s.metrics.up = json.get("upload.bandwidth")
            .expect("Ookla analyzer has no upload value")
            .to_u64();
    }
}

#[cfg(test)]
mod ookla_tests {
    use crate::OoklaAnalyzer;
    use speed_feed_lib::System;

    #[test]
    fn test_analyze() {
        let mut s = System::new();

        s.add(&OoklaAnalyzer {});

        s.run();

        println!("{:?}", s.metrics);

        assert_ne!(s.metrics.ping, 0);
        assert_ne!(s.metrics.up, 0);
        assert_ne!(s.metrics.down, 0);
    }
}