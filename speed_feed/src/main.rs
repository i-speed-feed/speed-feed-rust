use speed_feed_analyzer_ookla::OoklaAnalyzer;
use speed_feed_analyzer::Analyzer;

fn main() {
    let analyzer = OoklaAnalyzer::new();
    let metric = analyzer.analyze();
    println!("Metrics: \n  up: {}\n  down: {}\n  ping: {}", metric.up, metric.down, metric.ping)
}
