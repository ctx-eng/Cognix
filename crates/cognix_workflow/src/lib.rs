use cognix_agent_runtime::AgentRuntime;

#[derive(Debug, Clone, Default)]
pub struct WorkflowEngine {
    agent_runtime: Option<AgentRuntime>,
    steps: Vec<String>,
}

impl WorkflowEngine {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bind_agent_runtime(&mut self, runtime: AgentRuntime) {
        self.agent_runtime = Some(runtime);
    }

    pub fn enqueue(&mut self, step: impl Into<String>) {
        self.steps.push(step.into());
    }

    pub fn step_count(&self) -> usize {
        self.steps.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tracks_workflow_steps() {
        let mut engine = WorkflowEngine::new();
        engine.enqueue("plan");
        engine.enqueue("execute");

        assert_eq!(engine.step_count(), 2);
    }
}
