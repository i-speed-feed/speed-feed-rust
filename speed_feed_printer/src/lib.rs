use speed_feed_lib::{Plugin, System};

pub struct Printer {}

impl Plugin for Printer {
    fn name(&self) -> &'static str {
        "Simple Printer"
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
    use speed_feed_lib::System;
    use crate::Printer;

    #[test]
    fn it_works() {
        let mut s = System::new();

        s.add(&Printer{});

        s.run();
    }
}
