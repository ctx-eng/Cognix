use cognix_context::ContextPipeline;
use cognix_kernel::MetaKernel;

#[derive(Debug, Clone, Default)]
pub struct AgentRuntime {
    kernel: Option<MetaKernel>,
    context: Option<ContextPipeline>,
    steps: Vec<String>,
}

impl AgentRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bind_kernel(&mut self, kernel: MetaKernel) {
        self.kernel = Some(kernel);
    }

    pub fn bind_context(&mut self, context: ContextPipeline) {
        self.context = Some(context);
    }

    pub fn plan(&mut self, step: impl Into<String>) {
        let step = step.into();
        self.steps.push(step.clone());

        if let Some(kernel) = &mut self.kernel {
            kernel.publish_event("agent", "agent_runtime", step);
        }
    }

    pub fn execution_plan(&self) -> &[String] {
        &self.steps
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn records_planned_steps() {
        let mut runtime = AgentRuntime::new();
        runtime.plan("inspect workspace");
        runtime.plan("apply change");

        assert_eq!(runtime.execution_plan().len(), 2);
    }
}
