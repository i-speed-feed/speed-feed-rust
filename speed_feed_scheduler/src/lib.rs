use log::{warn, info};
use speed_feed_lib::{Plugin, System};
use std::thread;
use std::time::Duration;

pub struct SchedulerPlugin {}

const DEFAULT_INTERVAL: u64 = 120;

impl Plugin for SchedulerPlugin {
    fn name(&self) -> &'static str {
        "Speed :: Feed :: Scheduler"
    }

    fn prio(&self) -> u32 {
        100
    }

    fn run(&self, s: &mut System) {
        let interval = match s.config.get("scheduler.interval") {
            Some(interval_string) => {
                match interval_string.parse() {
                    Ok(interval) => interval,
                    _ => {
                        warn!("Value: {}, supplied for scheduler.interval cannot be parsed to a number.\n
                    Falling back to default.", interval_string);
                        DEFAULT_INTERVAL
                    }
                }
            }
            _ => {
                warn!("No value for scheduler.interval supplied. Falling back to default.");
                DEFAULT_INTERVAL
            }
        };

        info!("Scheduled run. {}s ETA", interval);
        thread::sleep(Duration::from_secs(interval));
    }
}

#[cfg(test)]
mod tests {

    use crate::SchedulerPlugin;
    use log::Level;
    use simple_logger;
    use speed_feed_lib::System;

    #[test]
    fn test_scheduler_plugin() {
        let _ = simple_logger::init_with_level(Level::Debug);

        let mut s: System = System::new();

        s.config.as_mut().insert(String::from("scheduler.interval"), String::from("20"));

        s.add(&SchedulerPlugin {});

        s.run();
    }
}
