use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Flow definition - represents the complete visual flow
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flow {
    pub nodes: Vec<Node>,
    pub connections: Vec<Connection>,
}

/// Node - represents a single block in the flow
/// Uses plugin-based architecture (Open/Closed Principle)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: String,
    /// Plugin type (e.g., "legacy-code", "debug")
    #[serde(rename = "type")]
    pub plugin_type: String,
    pub label: Option<String>,
    /// Dynamic properties from plugin definition
    #[serde(flatten)]
    pub properties: HashMap<String, serde_json::Value>,
}

/// Connection - represents data flow between nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub from: String, // Source node ID
    pub to: String,   // Target node ID
}
