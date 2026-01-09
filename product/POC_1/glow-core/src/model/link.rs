//! Link data model
//!
//! Links represent relationships between steps in the process.

use serde::{Deserialize, Serialize};

/// Built-in link types
pub const LINK_TYPE_DEPENDENCY: &str = "dependency";
pub const LINK_TYPE_PREDECESSOR: &str = "predecessor";

/// Link type definition
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct LinkType {
    /// Unique link type identifier
    pub id: String,
    /// Human-readable name
    #[serde(default)]
    pub name: Option<String>,
    /// Description of this link type
    #[serde(default)]
    pub description: Option<String>,
    /// Whether this link blocks step execution
    #[serde(default)]
    pub is_blocking: bool,
}

impl LinkType {
    /// Create the built-in dependency link type
    pub fn dependency() -> Self {
        Self {
            id: LINK_TYPE_DEPENDENCY.to_string(),
            name: Some("Dependency".to_string()),
            description: Some("Source step depends on target step completion".to_string()),
            is_blocking: true,
        }
    }

    /// Create the built-in predecessor link type
    pub fn predecessor() -> Self {
        Self {
            id: LINK_TYPE_PREDECESSOR.to_string(),
            name: Some("Predecessor".to_string()),
            description: Some("Target step should be done before source step".to_string()),
            is_blocking: true,
        }
    }
}

/// Link definition in process configuration
#[derive(Debug, Clone, Serialize, Deserialize, schemars::JsonSchema)]
pub struct LinkDefinition {
    /// Link type (dependency, predecessor, or custom)
    #[serde(default = "default_link_type")]
    pub r#type: String,
    /// Source step ID
    pub from: String,
    /// Target step ID
    pub to: String,
}

fn default_link_type() -> String {
    LINK_TYPE_DEPENDENCY.to_string()
}

impl LinkDefinition {
    /// Create a new dependency link
    pub fn dependency(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            r#type: LINK_TYPE_DEPENDENCY.to_string(),
            from: from.into(),
            to: to.into(),
        }
    }

    /// Create a new predecessor link
    pub fn predecessor(from: impl Into<String>, to: impl Into<String>) -> Self {
        Self {
            r#type: LINK_TYPE_PREDECESSOR.to_string(),
            from: from.into(),
            to: to.into(),
        }
    }

    /// Check if this is a blocking link type
    pub fn is_blocking(&self) -> bool {
        matches!(
            self.r#type.as_str(),
            LINK_TYPE_DEPENDENCY | LINK_TYPE_PREDECESSOR
        )
    }
}

/// Runtime link instance between steps
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Link {
    /// Link definition
    pub definition: LinkDefinition,
    /// Link type metadata
    pub link_type: LinkType,
    /// Whether the target step is complete
    pub is_satisfied: bool,
}

impl Link {
    /// Create a new link from definition
    pub fn from_definition(def: LinkDefinition, link_types: &[LinkType]) -> Self {
        let link_type = link_types
            .iter()
            .find(|lt| lt.id == def.r#type)
            .cloned()
            .unwrap_or_else(|| {
                if def.r#type == LINK_TYPE_DEPENDENCY {
                    LinkType::dependency()
                } else if def.r#type == LINK_TYPE_PREDECESSOR {
                    LinkType::predecessor()
                } else {
                    // Custom link type with default behavior
                    LinkType {
                        id: def.r#type.clone(),
                        name: None,
                        description: None,
                        is_blocking: false,
                    }
                }
            });

        Self {
            definition: def,
            link_type,
            is_satisfied: false,
        }
    }

    /// Check if this link is blocking and unsatisfied
    pub fn is_blocking(&self) -> bool {
        self.link_type.is_blocking && !self.is_satisfied
    }

    /// Get the source step ID
    pub fn from_step(&self) -> &str {
        &self.definition.from
    }

    /// Get the target step ID
    pub fn to_step(&self) -> &str {
        &self.definition.to
    }
}

/// Graph for detecting cycles in links
#[derive(Debug, Default)]
pub struct LinkGraph {
    /// Adjacency list: step_id -> dependent step_ids
    adjacency: std::collections::HashMap<String, Vec<String>>,
}

impl LinkGraph {
    /// Create a new empty graph
    pub fn new() -> Self {
        Self::default()
    }

    /// Add all links from definitions
    pub fn from_links(links: &[LinkDefinition]) -> Self {
        let mut graph = Self::new();
        for link in links {
            graph.add_edge(&link.from, &link.to);
        }
        graph
    }

    /// Add an edge from source to target
    pub fn add_edge(&mut self, from: &str, to: &str) {
        self.adjacency
            .entry(from.to_string())
            .or_default()
            .push(to.to_string());
    }

    /// Check if the graph contains a cycle
    pub fn has_cycle(&self) -> bool {
        self.find_cycle().is_some()
    }

    /// Find a cycle in the graph, returning the cycle path if found
    pub fn find_cycle(&self) -> Option<Vec<String>> {
        let mut visited = std::collections::HashSet::new();
        let mut rec_stack = std::collections::HashSet::new();
        let mut path = Vec::new();

        for node in self.adjacency.keys() {
            if self.dfs_cycle(node, &mut visited, &mut rec_stack, &mut path) {
                return Some(path);
            }
        }
        None
    }

    fn dfs_cycle(
        &self,
        node: &str,
        visited: &mut std::collections::HashSet<String>,
        rec_stack: &mut std::collections::HashSet<String>,
        path: &mut Vec<String>,
    ) -> bool {
        if !visited.contains(node) {
            visited.insert(node.to_string());
            rec_stack.insert(node.to_string());
            path.push(node.to_string());

            if let Some(neighbors) = self.adjacency.get(node) {
                for neighbor in neighbors {
                    if !visited.contains(neighbor)
                        && self.dfs_cycle(neighbor, visited, rec_stack, path)
                    {
                        return true;
                    } else if rec_stack.contains(neighbor) {
                        path.push(neighbor.clone());
                        return true;
                    }
                }
            }

            path.pop();
            rec_stack.remove(node);
        }
        false
    }

    /// Get all steps that the given step depends on (transitive)
    pub fn get_dependencies(&self, step_id: &str) -> Vec<String> {
        let mut deps = Vec::new();
        let mut visited = std::collections::HashSet::new();
        self.collect_dependencies(step_id, &mut deps, &mut visited);
        deps
    }

    fn collect_dependencies(
        &self,
        step_id: &str,
        deps: &mut Vec<String>,
        visited: &mut std::collections::HashSet<String>,
    ) {
        if let Some(neighbors) = self.adjacency.get(step_id) {
            for neighbor in neighbors {
                if visited.insert(neighbor.clone()) {
                    deps.push(neighbor.clone());
                    self.collect_dependencies(neighbor, deps, visited);
                }
            }
        }
    }

    /// Get all steps that depend on the given step (reverse)
    pub fn get_dependents(&self, step_id: &str) -> Vec<String> {
        self.adjacency
            .iter()
            .filter_map(|(from, tos)| {
                if tos.contains(&step_id.to_string()) {
                    Some(from.clone())
                } else {
                    None
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_link_definition_dependency() {
        let link = LinkDefinition::dependency("TASK-001", "REQ-001");
        assert_eq!(link.r#type, "dependency");
        assert_eq!(link.from, "TASK-001");
        assert_eq!(link.to, "REQ-001");
        assert!(link.is_blocking());
    }

    #[test]
    fn test_link_graph_no_cycle() {
        let links = vec![
            LinkDefinition::dependency("TASK-001", "REQ-001"),
            LinkDefinition::dependency("TASK-001", "REQ-002"),
            LinkDefinition::dependency("REQ-002", "REQ-001"),
        ];
        let graph = LinkGraph::from_links(&links);
        assert!(!graph.has_cycle());
    }

    #[test]
    fn test_link_graph_with_cycle() {
        let links = vec![
            LinkDefinition::dependency("A", "B"),
            LinkDefinition::dependency("B", "C"),
            LinkDefinition::dependency("C", "A"),
        ];
        let graph = LinkGraph::from_links(&links);
        assert!(graph.has_cycle());
    }

    #[test]
    fn test_link_graph_dependencies() {
        let links = vec![
            LinkDefinition::dependency("TASK", "REQ-001"),
            LinkDefinition::dependency("TASK", "REQ-002"),
            LinkDefinition::dependency("REQ-002", "REQ-001"),
        ];
        let graph = LinkGraph::from_links(&links);

        let deps = graph.get_dependencies("TASK");
        assert!(deps.contains(&"REQ-001".to_string()));
        assert!(deps.contains(&"REQ-002".to_string()));
    }

    #[test]
    fn test_link_graph_dependents() {
        let links = vec![
            LinkDefinition::dependency("TASK-001", "REQ-001"),
            LinkDefinition::dependency("TASK-002", "REQ-001"),
        ];
        let graph = LinkGraph::from_links(&links);

        let dependents = graph.get_dependents("REQ-001");
        assert!(dependents.contains(&"TASK-001".to_string()));
        assert!(dependents.contains(&"TASK-002".to_string()));
    }
}
