use crate::ir::{Flow, Connection};
use std::collections::{HashMap, HashSet, VecDeque};
use anyhow::{Result, anyhow};

/// Topological sort implementation for flow nodes
/// Follows Single Responsibility Principle - only handles graph ordering
pub struct TopologicalSort;

impl TopologicalSort {
    /// Sort nodes in execution order based on connections
    /// Returns node IDs in the order they should be executed
    pub fn sort(flow: &Flow) -> Result<Vec<String>> {
        let node_ids: HashSet<_> = flow.nodes.iter().map(|n| n.id.clone()).collect();
        
        // Build adjacency list (dependencies)
        let mut dependencies: HashMap<String, Vec<String>> = HashMap::new();
        let mut dependents: HashMap<String, Vec<String>> = HashMap::new();
        
        for node_id in &node_ids {
            dependencies.insert(node_id.clone(), Vec::new());
            dependents.insert(node_id.clone(), Vec::new());
        }
        
        for conn in &flow.connections {
            // 'to' node depends on 'from' node
            dependencies.get_mut(&conn.to)
                .ok_or_else(|| anyhow!("Unknown connection target: {}", conn.to))?
                .push(conn.from.clone());
                
            dependents.get_mut(&conn.from)
                .ok_or_else(|| anyhow!("Unknown connection source: {}", conn.from))?
                .push(conn.to.clone());
        }
        
        // Kahn's algorithm for topological sort
        let mut in_degree: HashMap<String, usize> = HashMap::new();
        for node_id in &node_ids {
            in_degree.insert(node_id.clone(), dependencies[node_id].len());
        }
        
        let mut queue: VecDeque<String> = VecDeque::new();
        for (node_id, &degree) in &in_degree {
            if degree == 0 {
                queue.push_back(node_id.clone());
            }
        }
        
        let mut sorted = Vec::new();
        
        while let Some(node_id) = queue.pop_front() {
            sorted.push(node_id.clone());
            
            if let Some(deps) = dependents.get(&node_id) {
                for dependent in deps {
                    let degree = in_degree.get_mut(dependent).unwrap();
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(dependent.clone());
                    }
                }
            }
        }
        
        // Check for cycles
        if sorted.len() != node_ids.len() {
            return Err(anyhow!("Cycle detected in flow graph"));
        }
        
        Ok(sorted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::Node;
    
    #[test]
    fn test_simple_sort() {
        let flow = Flow {
            nodes: vec![
                Node {
                    id: "a".to_string(),
                    plugin_type: "legacy-code".to_string(),
                    label: None,
                    properties: HashMap::new(),
                },
                Node {
                    id: "b".to_string(),
                    plugin_type: "debug".to_string(),
                    label: None,
                    properties: HashMap::new(),
                },
            ],
            connections: vec![
                Connection {
                    from: "a".to_string(),
                    to: "b".to_string(),
                    connection_type: Default::default(),
                    variable_mapping: None,
                },
            ],
        };
        
        let sorted = TopologicalSort::sort(&flow).unwrap();
        assert_eq!(sorted, vec!["a", "b"]);
    }
}
