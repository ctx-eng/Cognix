#[derive(Debug, Clone, Default)]
pub struct ReflectionEngine {
    evaluations: Vec<String>,
}

impl ReflectionEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record(&mut self, outcome: impl Into<String>) {
        self.evaluations.push(outcome.into());
    }

    pub fn evaluation_count(&self) -> usize {
        self.evaluations.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_outcomes() {
        let mut engine = ReflectionEngine::new();
        engine.record("success");
        engine.record("retry");

        assert_eq!(engine.evaluation_count(), 2);
    }
}
