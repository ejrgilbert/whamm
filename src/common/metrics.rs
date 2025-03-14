use std::collections::HashMap;
use std::time::{Duration, SystemTime};

#[derive(Default)]
pub struct Metrics {
    profiles: HashMap<String, Profile>
}
impl Metrics {
    pub fn start(&mut self, name: &String) {
        self.profiles.insert(name.clone(), Profile::start());
    }
    pub fn end(&mut self, name: &String) {
        if let Some(prof) = self.profiles.get_mut(name) {
            prof.end();
        } else {
            panic!("Unknown profile: {name}");
        }
    }
    pub fn flush(&self) {
        for (metric_name, prof) in &self.profiles {
            print!("{metric_name}\t: ");
            println!("{:?}", prof.elapsed())
        }
    }
}

#[derive(Default)]
struct Profile {
    start: Option<SystemTime>,
    end: Option<SystemTime>
}
impl Profile {
    pub(crate) fn start() -> Self {
        Self {
            start: Some(SystemTime::now()),
            end: None
        }
    }
    pub(crate) fn end(&mut self) {
        self.end = Some(SystemTime::now());
    }
    pub(crate) fn elapsed(&self) -> Duration {
        if let (Some(start), Some(end)) = (self.start, self.end) {
            match end.duration_since(start) {
                Ok(dur) => dur,
                Err(e) => panic!("Could not count duration of system time: {:?}", e)
            }
        } else {
            panic!("must have start and end times")
        }
    }
}