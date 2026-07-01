#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct KernelState {
    pub running: bool,
    pub event_count: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EventEnvelope {
    pub topic: String,
    pub source: String,
    pub payload: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct EventBus {
    events: Vec<EventEnvelope>,
}

impl EventBus {
    pub fn publish(&mut self, topic: impl Into<String>, source: impl Into<String>, payload: impl Into<String>) {
        self.events.push(EventEnvelope {
            topic: topic.into(),
            source: source.into(),
            payload: payload.into(),
        });
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn drain(&mut self) -> Vec<EventEnvelope> {
        std::mem::take(&mut self.events)
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Resource {
    pub id: String,
    pub kind: String,
    pub name: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct MetaKernel {
    pub name: String,
    pub capabilities: Vec<String>,
    pub state: KernelState,
    pub resources: Vec<Resource>,
    pub event_bus: EventBus,
}

impl MetaKernel {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            capabilities: Vec::new(),
            state: KernelState::default(),
            resources: Vec::new(),
            event_bus: EventBus::default(),
        }
    }

    pub fn register_capability(&mut self, capability: impl Into<String>) {
        let capability = capability.into();
        if !self.capabilities.contains(&capability) {
            self.capabilities.push(capability);
        }
    }

    pub fn register_resource(&mut self, resource: Resource) {
        if !self.resources.iter().any(|existing| existing.id == resource.id) {
            self.resources.push(resource);
        }
    }

    pub fn publish_event(&mut self, topic: impl Into<String>, source: impl Into<String>, payload: impl Into<String>) {
        self.event_bus.publish(topic, source, payload);
        self.state.event_count += 1;
    }

    pub fn pending_events(&self) -> usize {
        self.event_bus.event_count()
    }

    pub fn drain_events(&mut self) -> Vec<EventEnvelope> {
        self.event_bus.drain()
    }

    pub fn start(&mut self) {
        self.state.running = true;
        self.publish_event("kernel", "meta_kernel", "started");
    }

    pub fn stop(&mut self) {
        self.state.running = false;
        self.publish_event("kernel", "meta_kernel", "stopped");
    }

    pub fn is_running(&self) -> bool {
        self.state.running
    }

    pub fn capability_count(&self) -> usize {
        self.capabilities.len()
    }

    pub fn resource_count(&self) -> usize {
        self.resources.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registers_capabilities_and_starts_kernel() {
        let mut kernel = MetaKernel::new("cognix");
        kernel.register_capability("context");
        kernel.register_capability("agent");
        kernel.start();

        assert!(kernel.is_running());
        assert_eq!(kernel.capability_count(), 2);
        assert_eq!(kernel.state.event_count, 1);
    }

    #[test]
    fn publishes_events_and_tracks_resources() {
        let mut kernel = MetaKernel::new("cognix");
        kernel.register_resource(Resource {
            id: "workspace".into(),
            kind: "workspace".into(),
            name: "root".into(),
        });
        kernel.publish_event("context", "context_pipeline", "workspace:src");

        assert_eq!(kernel.resource_count(), 1);
        assert_eq!(kernel.pending_events(), 1);
        let events = kernel.drain_events();
        assert_eq!(events[0].topic, "context");
    }
}
