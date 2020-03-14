use speed_feed_lib::{Plugin, System};

pub struct Printer {}

impl Plugin for Printer {
    fn name(&self) -> &'static str {
        "Speed :: Feed :: Printer"
    }

    fn prio(&self) -> u32 {
        0
    }

    fn run(&self, s: &mut System) {
        println!("Ping: {}", s.metrics.ping);
        println!("Up: {}", s.metrics.up);
        println!("Down: {}", s.metrics.down);
    }
}

#[cfg(test)]
mod tests {
    use crate::Printer;
    use log::Level;
    use simple_logger;
    use speed_feed_lib::System;

    #[test]
    fn test_printer_plugin() {
        let _ = simple_logger::init_with_level(Level::Debug);

        let mut s = System::new();

        s.add(&Printer {});

        s.run();
    }
}
