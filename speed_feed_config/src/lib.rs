use eclectic::Collection;
use log::{info, warn};
use speed_feed_lib::{Plugin, System};
use std::collections::HashMap;
use std::env::current_dir;
extern crate yaml_rust;

pub struct YamlConfig {}

impl Plugin for YamlConfig {
    fn name(&self) -> &'static str {
        "Speed :: Feed :: Config"
    }

    fn prio(&self) -> u32 {
        100
    }

    fn run(&self, s: &mut System) {
        let mut settings = config::Config::default();

        let _ = settings.merge(config::Environment::with_prefix("SPEED_FEED"));

        let mut workdir = String::from(current_dir().unwrap().as_os_str().to_str().unwrap());
        workdir.push_str("/config.yaml");

        match settings.merge(config::File::with_name(workdir.as_str()).required(false)) {
            Ok(_config) => info!("Loaded: {}", workdir),
            Err(err) => warn!("{}", err),
        }

        match s.config.get("config") {
            Some(config_loc) => {
                match settings.merge(config::File::with_name(config_loc).required(false)) {
                    Ok(_config) => info!("Loaded: {}", config_loc),
                    Err(err) => warn!("{}", err),
                }
            }
            _ => {}
        }

        s.config
            .append(&mut settings.try_into::<HashMap<String, String>>().unwrap());
    }
}

#[cfg(test)]
mod tests {
    use crate::YamlConfig;
    use speed_feed_lib::System;
    use std::env::*;
    use std::path::PathBuf;

    #[test]
    fn test_config_plugin() {
        simple_logger::init().unwrap();

        let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        d.push("resources/test/config.yaml");

        set_var("SPEED_FEED_TEST_BLA", "VALUE");

        let mut s = System::new();

        s.config
            .insert(String::from("config"), String::from(d.to_str().unwrap()));

        s.add(&YamlConfig {});

        s.run();

        println!("{:?}", s.config);

        assert_eq!(s.config.get("another.prop").unwrap(), "with-value");
        assert_eq!(s.config.get("test-prop").unwrap(), "test-value");
    }
}
