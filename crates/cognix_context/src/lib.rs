use cognix_kernel::MetaKernel;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextSignal {
    pub kind: String,
    pub value: String,
}

#[derive(Debug, Clone, Default)]
pub struct ContextPipeline {
    kernel: Option<MetaKernel>,
    signals: Vec<ContextSignal>,
}

impl ContextPipeline {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn bind_kernel(&mut self, kernel: MetaKernel) {
        self.kernel = Some(kernel);
    }

    pub fn ingest(&mut self, kind: impl Into<String>, value: impl Into<String>) {
        let kind = kind.into();
        let value = value.into();

        self.signals.push(ContextSignal {
            kind: kind.clone(),
            value: value.clone(),
        });

        if let Some(kernel) = &mut self.kernel {
            kernel.publish_event("context", "context_pipeline", format!("{kind}:{value}"));
        }
    }

    pub fn compile(&self) -> Vec<String> {
        self.signals
            .iter()
            .map(|signal| format!("{}:{}", signal.kind, signal.value))
            .collect()
    }

    pub fn kernel_name(&self) -> Option<&str> {
        self.kernel.as_ref().map(|kernel| kernel.name.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compiles_signals_into_context_view() {
        let mut pipeline = ContextPipeline::new();
        pipeline.ingest("workspace", "src");
        pipeline.ingest("intent", "refactor");

        let context = pipeline.compile();
        assert_eq!(context.len(), 2);
        assert!(context.contains(&"workspace:src".to_string()));
    }
}
