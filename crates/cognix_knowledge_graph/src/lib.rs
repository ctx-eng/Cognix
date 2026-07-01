use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeNode {
    pub id: String,
    pub kind: String,
    pub label: String,
    pub metadata: BTreeMap<String, String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnowledgeEdge {
    pub from: String,
    pub to: String,
    pub relation: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct KnowledgeGraph {
    nodes: Vec<KnowledgeNode>,
    edges: Vec<KnowledgeEdge>,
}

impl KnowledgeGraph {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, node: KnowledgeNode) {
        if !self.nodes.iter().any(|existing| existing.id == node.id) {
            self.nodes.push(node);
        }
    }

    pub fn add_edge(&mut self, edge: KnowledgeEdge) {
        if !self.edges.iter().any(|existing| {
            existing.from == edge.from && existing.to == edge.to && existing.relation == edge.relation
        }) {
            self.edges.push(edge);
        }
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn related_nodes(&self, node_id: &str) -> BTreeSet<&str> {
        let mut related = BTreeSet::new();
        for edge in &self.edges {
            if edge.from == node_id {
                related.insert(edge.to.as_str());
            } else if edge.to == node_id {
                related.insert(edge.from.as_str());
            }
        }
        related
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stores_relationships() {
        let mut graph = KnowledgeGraph::new();
        graph.add_node(KnowledgeNode {
            id: "workspace".into(),
            kind: "resource".into(),
            label: "Workspace".into(),
            metadata: BTreeMap::new(),
        });
        graph.add_node(KnowledgeNode {
            id: "project".into(),
            kind: "resource".into(),
            label: "Project".into(),
            metadata: BTreeMap::new(),
        });
        graph.add_edge(KnowledgeEdge {
            from: "workspace".into(),
            to: "project".into(),
            relation: "contains".into(),
        });

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);
        assert!(graph.related_nodes("workspace").contains("project"));
    }
}
