#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyRule {
    pub name: String,
    pub allows: bool,
}

#[derive(Debug, Clone, Default)]
pub struct PolicyEngine {
    rules: Vec<PolicyRule>,
}

impl PolicyEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_rule(&mut self, rule: PolicyRule) {
        self.rules.push(rule);
    }

    pub fn allows(&self, name: &str) -> bool {
        self.rules
            .iter()
            .find(|rule| rule.name == name)
            .map(|rule| rule.allows)
            .unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn evaluates_rules() {
        let mut engine = PolicyEngine::new();
        engine.add_rule(PolicyRule {
            name: "remote_exec".into(),
            allows: true,
        });

        assert!(engine.allows("remote_exec"));
        assert!(!engine.allows("secret_read"));
    }
}
