use cognix_agent_runtime::AgentRuntime;
use cognix_context::ContextPipeline;
use cognix_kernel::{MetaKernel, Resource};
use cognix_policy::PolicyEngine;
use cognix_reflection::ReflectionEngine;
use cognix_runtime::SelfEvolvingRuntime;
use cognix_workflow::WorkflowEngine;

pub fn initialize_runtime() {
    let mut kernel = MetaKernel::new("cognix");
    kernel.register_capability("context");
    kernel.register_capability("agent");
    kernel.register_capability("workflow");
    kernel.register_resource(Resource {
        id: "workspace".into(),
        kind: "workspace".into(),
        name: "default".into(),
    });
    kernel.start();

    let mut context = ContextPipeline::new();
    context.bind_kernel(kernel.clone());
    context.ingest("workspace", "app");

    let mut agent_runtime = AgentRuntime::new();
    agent_runtime.bind_kernel(kernel.clone());
    agent_runtime.bind_context(context.clone());
    agent_runtime.plan("bootstrap runtime");

    let mut workflow_engine = WorkflowEngine::new();
    workflow_engine.bind_agent_runtime(agent_runtime.clone());
    workflow_engine.enqueue("initialize");

    let mut policy_engine = PolicyEngine::new();
    policy_engine.add_rule(cognix_policy::PolicyRule {
        name: "remote_exec".into(),
        allows: true,
    });

    let mut reflection_engine = ReflectionEngine::new();
    reflection_engine.record("runtime_bootstrap");

    let mut runtime = SelfEvolvingRuntime::new();
    runtime.bind_kernel(kernel);
    runtime.bind_context(context);
    runtime.bind_agent_runtime(agent_runtime);
    runtime.bind_workflow_engine(workflow_engine);
    runtime.bind_policy_engine(policy_engine);
    runtime.bind_reflection_engine(reflection_engine);

    let _ = runtime;
}
