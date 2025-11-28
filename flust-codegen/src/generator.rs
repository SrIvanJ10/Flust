use flust_core::ir::Flow;
use flust_core::topological_sort::TopologicalSort;
use crate::template_engine::TemplateEngine;
use anyhow::{Result, anyhow};
use std::collections::HashMap;

/// Plugin template definitions
/// In a production system, these would be loaded from plugin files
/// Follows Dependency Inversion Principle - depends on abstractions
const LEGACY_CODE_TEMPLATE: &str = "{{code}}";
const DEBUG_TEMPLATE: &str = r#"{{#if label}}println!("{{label}}: {:?}", {{variable}});
{{else}}println!("{:?}", {{variable}});
{{/if}}"#;

/// Code generator - converts Flow IR to Rust code
/// Follows Open/Closed Principle via template-based generation
pub fn generate_rust(flow: &Flow) -> Result<String> {
    if flow.nodes.is_empty() {
        return Ok("fn main() {\n    // Empty flow\n}\n".to_string());
    }
    
    // Sort nodes in execution order
    let sorted_ids = TopologicalSort::sort(flow)?;
    
    // Build a lookup map for quick node access
    let node_map: HashMap<_, _> = flow.nodes.iter()
        .map(|n| (n.id.clone(), n))
        .collect();
    
    let mut code = String::new();
    
    // Generate main function header
    code.push_str("fn main() {\n");
    
    // Generate code for each node in topological order
    for node_id in sorted_ids {
        let node = node_map.get(&node_id)
            .ok_or_else(|| anyhow!("Node {} not found", node_id))?;
        
        let node_code = generate_node_code(node)?;
        
        // Add indentation and append to main code
        for line in node_code.lines() {
            if !line.trim().is_empty() {
                code.push_str("    ");
                code.push_str(line);
                code.push('\n');
            }
        }
    }
    
    code.push_str("}\n");
    
    Ok(code)
}

/// Generate code for a single node based on its plugin type
/// Follows Single Responsibility Principle
fn generate_node_code(node: &flust_core::ir::Node) -> Result<String> {
    // Get the template for this plugin type
    let template = match node.plugin_type.as_str() {
        "legacy-code" | "legacy_code" => LEGACY_CODE_TEMPLATE,
        "debug" => DEBUG_TEMPLATE,
        _ => return Err(anyhow!("Unknown plugin type: {}", node.plugin_type)),
    };
    
    // Build context from node properties
    let mut context = HashMap::new();
    
    for (key, value) in &node.properties {
        // Convert JSON value to string
        let value_str = match value {
            serde_json::Value::String(s) => s.clone(),
            serde_json::Value::Number(n) => n.to_string(),
            serde_json::Value::Bool(b) => b.to_string(),
            serde_json::Value::Null => String::new(),
            _ => value.to_string(),
        };
        
        context.insert(key.clone(), value_str);
    }
    
    // Render the template
    let code = TemplateEngine::render(template, &context);
    
    Ok(code)
}

#[cfg(test)]
mod tests {
    use super::*;
    use flust_core::ir::{Node, Connection};
    use serde_json::json;
    
    #[test]
    fn test_legacy_code_generation() {
        let mut properties = HashMap::new();
        properties.insert("code".to_string(), json!("let x = 42;"));
        
        let node = Node {
            id: "node1".to_string(),
            plugin_type: "legacy-code".to_string(),
            label: Some("Test".to_string()),
            properties,
        };
        
        let flow = Flow {
            nodes: vec![node],
            connections: vec![],
        };
        
        let code = generate_rust(&flow).unwrap();
        assert!(code.contains("let x = 42;"));
        assert!(code.contains("fn main()"));
    }
    
    #[test]
    fn test_debug_generation() {
        let mut properties = HashMap::new();
        properties.insert("variable".to_string(), json!("x"));
        properties.insert("label".to_string(), json!("Value"));
        
        let node = Node {
            id: "node1".to_string(),
            plugin_type: "debug".to_string(),
            label: Some("Debug".to_string()),
            properties,
        };
        
        let flow = Flow {
            nodes: vec![node],
            connections: vec![],
        };
        
        let code = generate_rust(&flow).unwrap();
        assert!(code.contains(r#"println!("Value: {:?}", x);"#));
    }
    
    #[test]
    fn test_connected_nodes() {
        let mut prop1 = HashMap::new();
        prop1.insert("code".to_string(), json!("let result = 100;"));
        
        let mut prop2 = HashMap::new();
        prop2.insert("variable".to_string(), json!("result"));
        prop2.insert("label".to_string(), json!(""));
        
        let flow = Flow {
            nodes: vec![
                Node {
                    id: "a".to_string(),
                    plugin_type: "legacy-code".to_string(),
                    label: None,
                    properties: prop1,
                },
                Node {
                    id: "b".to_string(),
                    plugin_type: "debug".to_string(),
                    label: None,
                    properties: prop2,
                },
            ],
            connections: vec![
                Connection {
                    from: "a".to_string(),
                    to: "b".to_string(),
                },
            ],
        };
        
        let code = generate_rust(&flow).unwrap();
        
        // Check that code appears in correct order
        let result_pos = code.find("let result = 100;").unwrap();
        let println_pos = code.find("println!").unwrap();
        assert!(result_pos < println_pos, "Code should be in topological order");
    }
}
