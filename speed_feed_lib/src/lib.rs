use std::collections::HashMap;
use log::info;

#[derive(Debug)]
pub struct Metrics {
    pub down: u64,
    pub up: u64,
    pub ping: u64,
}

pub trait Printable {
    fn print(&self);
}

pub trait Plugin: Printable {
    fn name(&self) -> &'static str;
    fn prio(&self) -> u32;
    fn run(&self, s: &mut System);
}

impl<T> Printable for T where T: Plugin {
    fn print(&self) {
        println!("{}", self.name())
    }
}

pub struct System<'l> {
    plugins: Box<Vec<&'l dyn Plugin>>,
    pub config: Box<HashMap<String, String>>,
    pub metrics: Box<Metrics>,
}

impl<'l> Printable for System<'l> {
    fn print(&self) {
        self.plugins.iter().for_each(|printable|
            printable.print()
        )
    }
}

impl<'l> System<'l> {
    pub fn new() -> System<'l> {
        System {
            plugins: Box::new(Vec::new()),
            config: Box::new(HashMap::new()),
            metrics: Box::new(Metrics { ping: 0, up: 0, down: 0 }),
        }
    }

    pub fn run(&mut self) {
        self.plugins.clone().iter_mut().for_each(|plugin| {
            info!("Running: {}", plugin.name());
            plugin.run(self);
        });
    }

    pub fn add(&mut self, p: &'l dyn Plugin) {
        self.plugins.insert(self.find_spot(p.prio()), p);
    }

    fn find_spot(&self, prio: u32) -> usize {
        self.plugins.iter().position(|plugin|
            plugin.prio() < prio
        ).unwrap_or(self.plugins.len())
    }
}

#[cfg(test)]
mod tests {
    use std::any::Any;
    use crate::{System, Printable, Metrics, Plugin};

    struct SimplePlugin {
        x: &'static str,
        prio: u32,
    }

    impl Plugin for SimplePlugin {
        fn name(&self) -> &'static str {
            self.x
        }

        fn prio(&self) -> u32 {
            self.prio
        }

        fn run(&self, s: &mut System) {
            s.metrics.ping = 10;
            s.metrics.down = 50000;
            s.metrics.up = 10000;
        }
    }

    #[test]
    fn test_system() {
        let mut s = System::new();

        s.add(&SimplePlugin { x: "bla", prio: 9 });

        s.run();

        s.print();

        assert_eq!(s.metrics.ping, 10);
        assert_eq!(s.metrics.up, 10000);
        assert_eq!(s.metrics.down, 50000);
    }
}
