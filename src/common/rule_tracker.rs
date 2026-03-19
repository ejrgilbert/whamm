#[derive(Clone, Debug, Default)]
pub struct RuleTracker(Option<String>);

impl RuleTracker {
    pub fn push(&mut self, segment: &str) {
        match self.0.take() {
            None => self.0 = Some(segment.to_string()),
            Some(mut s) => {
                s += segment;
                self.0 = Some(s);
            }
        }
    }

    pub fn pop(&mut self) {
        self.0 = self
            .0
            .take()
            .and_then(|rule| rule.rfind(':').map(|idx| rule[..idx].to_string()));
    }

    pub fn get_opt_owned(&self) -> Option<String> {
        self.0.clone()
    }

    pub fn get_owned(&self) -> String {
        self.0
            .clone()
            .expect("RuleTracker: expected an active rule but found None")
    }
}
