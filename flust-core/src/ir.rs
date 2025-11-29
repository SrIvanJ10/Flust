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
    pub plugin_type: String,
    pub label: Option<String>,
    /// Dynamic properties from plugin definition
    pub properties: HashMap<String, serde_json::Value>,
    /// Parent node ID for hierarchical structures (e.g. function containers)
    pub parent_id: Option<String>,
}

/// Connection type - defines how the connection behaves
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ConnectionType {
    Simple,
    FunctionCall,
}

impl Default for ConnectionType {
    fn default() -> Self {
        ConnectionType::Simple
    }
}

/// Connection - represents data flow between nodes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Connection {
    pub from: String, // Source node ID
    pub to: String,   // Target node ID
    #[serde(default)]
    pub connection_type: ConnectionType,
    /// Maps argument name to variable name for function calls
    pub variable_mapping: Option<HashMap<String, String>>,
}
