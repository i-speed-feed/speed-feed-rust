use speed_feed_analyzer::{Analyzer, Metrics};

pub struct OoklaAnalyzer {
    cli: String
}

impl OoklaAnalyzer {
    pub fn new() -> OoklaAnalyzer {
        return OoklaAnalyzer { cli: String::from("ookla-cli") };
    }
}

impl Analyzer for OoklaAnalyzer {
    fn analyze(&self) -> Metrics {
        return Metrics { down: 0.0, up: 0.0, ping: 0 };
    }
}

#[cfg(test)]
mod ookla_tests {
    use speed_feed_analyzer::{Analyzer};
    use crate::OoklaAnalyzer;

    #[test]
    fn test_run() {
        let analyzer = OoklaAnalyzer::new();
        let metrics = analyzer.analyze();

        assert_eq!(metrics.ping, 0);
        assert_eq!(metrics.up, 0.0);
        assert_eq!(metrics.down, 0.0);
    }
}