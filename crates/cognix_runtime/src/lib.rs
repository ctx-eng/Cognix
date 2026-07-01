use cognix_agent_runtime::AgentRuntime;
use cognix_context::ContextPipeline;
use cognix_kernel::MetaKernel;
use cognix_policy::PolicyEngine;
use cognix_reflection::ReflectionEngine;
use cognix_workflow::WorkflowEngine;

#[derive(Debug, Clone, Default)]
pub struct SelfEvolvingRuntime {
    kernel: Option<MetaKernel>,
    context: Option<ContextPipeline>,
    agent_runtime: Option<AgentRuntime>,
    workflow_engine: Option<WorkflowEngine>,
    policy_engine: Option<PolicyEngine>,
    reflection_engine: Option<ReflectionEngine>,
}

impl SelfEvolvingRuntime {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bind_kernel(&mut self, kernel: MetaKernel) {
        self.kernel = Some(kernel);
    }

    pub fn bind_context(&mut self, context: ContextPipeline) {
        self.context = Some(context);
    }

    pub fn bind_agent_runtime(&mut self, runtime: AgentRuntime) {
        self.agent_runtime = Some(runtime);
    }

    pub fn bind_workflow_engine(&mut self, engine: WorkflowEngine) {
        self.workflow_engine = Some(engine);
    }

    pub fn bind_policy_engine(&mut self, engine: PolicyEngine) {
        self.policy_engine = Some(engine);
    }

    pub fn bind_reflection_engine(&mut self, engine: ReflectionEngine) {
        self.reflection_engine = Some(engine);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn binds_runtime_components() {
        let mut runtime = SelfEvolvingRuntime::new();
        runtime.bind_kernel(MetaKernel::new("cognix"));
        runtime.bind_context(ContextPipeline::new());
        runtime.bind_agent_runtime(AgentRuntime::new());
        runtime.bind_workflow_engine(WorkflowEngine::new());
        runtime.bind_policy_engine(PolicyEngine::new());
        runtime.bind_reflection_engine(ReflectionEngine::new());

        assert!(runtime.kernel.is_some());
        assert!(runtime.context.is_some());
    }
}
