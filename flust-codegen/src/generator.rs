use flust_core::ir::{Flow, Node, Connection};
use flust_core::topological_sort::TopologicalSort;
use crate::template_engine::TemplateEngine;
use anyhow::{Result, anyhow};
use std::collections::{HashMap, HashSet};

const LEGACY_CODE_TEMPLATE: &str = "{{code}}";
const DEBUG_TEMPLATE: &str = r#"{{#if label}}println!("{{label}}: {:?}", {{variable}});
{{else}}println!("{:?}", {{variable}});
{{/if}}"#;

/// Code generator - converts Flow IR to Rust code
pub fn generate_rust(flow: &Flow) -> Result<String> {
    // 1. Index nodes by Parent ID
    let mut nodes_by_parent: HashMap<Option<String>, Vec<&Node>> = HashMap::new();
    let mut node_map: HashMap<String, &Node> = HashMap::new();
    
    for node in &flow.nodes {
        node_map.insert(node.id.clone(), node);
        nodes_by_parent.entry(node.parent_id.clone()).or_default().push(node);
    }

    // 2. Identify Function Definitions
    let function_defs: Vec<&Node> = flow.nodes.iter()
        .filter(|n| n.plugin_type == "function-definition")
        .collect();

    let mut code = String::new();

    // 3. Generate Functions
    for func_def in &function_defs {
        let func_name = func_def.properties.get("function_name")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
            
        if func_name == "main" {
            continue; // Skip main here, handled separately
        }
        
        // Generate arguments signature
        let arguments = func_def.properties.get("arguments")
            .and_then(|v| v.as_array())
            .map(|arr| arr.to_vec())
            .unwrap_or_default();
            
        let mut args_str = Vec::new();
        for arg in arguments {
            let name = arg.get("name").and_then(|v| v.as_str()).unwrap_or("arg");
            let type_ = arg.get("type").and_then(|v| v.as_str()).unwrap_or("i32");
            args_str.push(format!("{}: {}", name, type_));
        }
        
        // Get return type if specified
        let return_type = func_def.properties.get("return_type")
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty());
        
        // Generate body
        let children = nodes_by_parent.get(&Some(func_def.id.clone()))
            .map(|v| v.as_slice())
            .unwrap_or(&[]);
            
        let body = generate_scope_code(children, &flow.connections, &node_map)?;
        
        // Generate function signature with optional return type
        if let Some(ret_type) = return_type {
            code.push_str(&format!("async fn {}({}) -> {} {{\n{}\n}}\n\n", func_name, args_str.join(", "), ret_type, body));
        } else {
            code.push_str(&format!("async fn {}({}) {{\n{}\n}}\n\n", func_name, args_str.join(", "), body));
        }
    }

    // 4. Generate Main
    let main_def = function_defs.iter().find(|n| 
        n.properties.get("function_name").and_then(|v| v.as_str()) == Some("main")
    );
    
    let main_children = if let Some(main) = main_def {
        nodes_by_parent.get(&Some(main.id.clone()))
            .map(|v| v.as_slice())
            .unwrap_or(&[])
            .to_vec()
    } else {
        // Fallback: use root nodes (parent_id == None) that are NOT function definitions
        nodes_by_parent.get(&None)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
            .iter()
            .filter(|n| n.plugin_type != "function-definition")
            .cloned()
            .collect()
    };
    
    let main_body = generate_scope_code(&main_children, &flow.connections, &node_map)?;
    
    code.push_str("#[tokio::main]\nasync fn main() {\n");
    code.push_str(&main_body);
    code.push_str("}\n");

    Ok(code)
}

fn generate_scope_code(nodes: &[&Node], connections: &[Connection], node_map: &HashMap<String, &Node>) -> Result<String> {
    if nodes.is_empty() {
        return Ok(String::new());
    }

    // Create sub-flow for topological sort
    let child_ids: HashSet<String> = nodes.iter().map(|n| n.id.clone()).collect();
    let relevant_connections: Vec<Connection> = connections.iter()
        .filter(|c| child_ids.contains(&c.from) && child_ids.contains(&c.to))
        .cloned()
        .collect();
        
    let sub_flow = Flow {
        nodes: nodes.iter().map(|&n| n.clone()).collect(),
        connections: relevant_connections.clone(),
    };
    
    let sorted_ids = TopologicalSort::sort(&sub_flow)?;
    
    // Map incoming connections for each node (for variable mapping)
    let incoming_connections: HashMap<String, Vec<&Connection>> = connections.iter()
        .fold(HashMap::new(), |mut acc, conn| {
            acc.entry(conn.to.clone()).or_default().push(conn);
            acc
        });

    let mut scope_code = String::new();
    
    for node_id in sorted_ids {
        let node = node_map.get(&node_id).ok_or_else(|| anyhow!("Node not found"))?;
        
        // Skip start-node (it just starts the flow) and function-definition (containers)
        if node.plugin_type == "start-node" || node.plugin_type == "function-definition" {
            continue;
        }
        
        let incoming = incoming_connections.get(&node_id).map(|v| v.as_slice()).unwrap_or(&[]);
        let node_code = generate_node_code(node, incoming)?;
        
        for line in node_code.lines() {
            if !line.trim().is_empty() {
                scope_code.push_str("    ");
                scope_code.push_str(line);
                scope_code.push('\n');
            }
        }
    }
    
    Ok(scope_code)
}

fn generate_node_code(node: &Node, incoming_connections: &[&Connection]) -> Result<String> {
    match node.plugin_type.as_str() {
        "call-function" => {
            let func_name = node.properties.get("target_function")
                .and_then(|v| v.as_str())
                .ok_or_else(|| anyhow!("Call function missing target_function"))?;
                
            let arguments = node.properties.get("arguments")
                .and_then(|v| v.as_array())
                .map(|arr| arr.to_vec())
                .unwrap_or_default();
                
            // Find variable mapping from incoming connection
            // We assume the first incoming connection carries the mapping
            // In a more complex flow, we might need to merge mappings or check specific connection
            let mapping = incoming_connections.first()
                .and_then(|c| c.variable_mapping.as_ref())
                .ok_or_else(|| anyhow!("Call function '{}' missing incoming connection with variable mapping", func_name))?;
                
            let mut call_args = Vec::new();
            for arg in arguments {
                let arg_name = arg.get("name").and_then(|v| v.as_str()).unwrap_or("");
                let var_name = mapping.get(arg_name)
                    .ok_or_else(|| anyhow!("Argument '{}' not mapped", arg_name))?;
                call_args.push(var_name.as_str());
            }
            
            // Check if there's a return variable
            let return_var = node.properties.get("return_variable")
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty());
            
            // Check if there's a return type
            let return_type = node.properties.get("return_type")
                .and_then(|v| v.as_str())
                .filter(|s| !s.is_empty());
            
            // Check if we should declare a new variable (default: true)
            let declare_variable = node.properties.get("declare_variable")
                .and_then(|v| v.as_bool())
                .unwrap_or(true);
            
            // Check if variable should be mutable (default: false)
            let is_mutable = node.properties.get("is_mutable")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            
            if let Some(var_name) = return_var {
                let mut_keyword = if is_mutable { "mut " } else { "" };
                let function_call = format!("{}({}).await", func_name, call_args.join(", "));
                
                if declare_variable {
                    // Declare new variable: let [mut] var[: Type] = ...
                    if let Some(ret_type) = return_type {
                        Ok(format!("let {}{}: {} = {};", mut_keyword, var_name, ret_type, function_call))
                    } else {
                        Ok(format!("let {}{} = {};", mut_keyword, var_name, function_call))
                    }
                } else {
                    // Assign to existing variable: var = ...
                    Ok(format!("{} = {};", var_name, function_call))
                }
            } else {
                Ok(format!("{}({}).await;", func_name, call_args.join(", ")))
            }
        },
        "legacy-code" | "legacy_code" => {
            let mut context = HashMap::new();
            for (k, v) in &node.properties {
                // Use as_str() to preserve actual newlines instead of escaped \n
                let value = if let Some(s) = v.as_str() {
                    s.to_string()
                } else {
                    v.to_string().trim_matches('"').to_string()
                };
                context.insert(k.clone(), value);
            }
            Ok(TemplateEngine::render(LEGACY_CODE_TEMPLATE, &context))
        },
        "debug" => {
            let mut context = HashMap::new();
            for (k, v) in &node.properties {
                // Use as_str() to preserve actual newlines instead of escaped \n
                let value = if let Some(s) = v.as_str() {
                    s.to_string()
                } else {
                    v.to_string().trim_matches('"').to_string()
                };
                context.insert(k.clone(), value);
            }
            Ok(TemplateEngine::render(DEBUG_TEMPLATE, &context))
        },
        _ => Ok(format!("// Unknown plugin: {}", node.plugin_type)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use flust_core::ir::{Node, Connection, ConnectionType};
    use serde_json::json;
    
    #[test]
    fn test_hierarchical_generation() {
        // 1. Define Function Container
        let mut func_props = HashMap::new();
        func_props.insert("function_name".to_string(), json!("my_func"));
        func_props.insert("arguments".to_string(), json!([{"name": "x", "type": "i32"}]));
        
        let func_container = Node {
            id: "func_container".to_string(),
            plugin_type: "function-definition".to_string(),
            label: Some("My Func".to_string()),
            properties: func_props,
            parent_id: None,
        };
        
        // 2. Define Start Node inside container
        let start_node = Node {
            id: "start".to_string(),
            plugin_type: "start-node".to_string(),
            label: Some("Start".to_string()),
            properties: HashMap::new(),
            parent_id: Some("func_container".to_string()),
        };
        
        // 3. Define Logic inside container (Debug)
        let mut debug_props = HashMap::new();
        debug_props.insert("variable".to_string(), json!("x"));
        
        let debug_node = Node {
            id: "debug".to_string(),
            plugin_type: "debug".to_string(),
            label: Some("Debug".to_string()),
            properties: debug_props,
            parent_id: Some("func_container".to_string()),
        };
        
        // Connection Start -> Debug
        let conn1 = Connection {
            from: "start".to_string(),
            to: "debug".to_string(),
            connection_type: ConnectionType::Simple,
            variable_mapping: None,
        };
        
        // 4. Define Main Container
        let mut main_props = HashMap::new();
        main_props.insert("function_name".to_string(), json!("main"));
        
        let main_container = Node {
            id: "main_container".to_string(),
            plugin_type: "function-definition".to_string(),
            label: Some("Main".to_string()),
            properties: main_props,
            parent_id: None,
        };
        
        // 5. Define Start Node inside Main
        let main_start = Node {
            id: "main_start".to_string(),
            plugin_type: "start-node".to_string(),
            label: Some("Start".to_string()),
            properties: HashMap::new(),
            parent_id: Some("main_container".to_string()),
        };
        
        // 6. Define Call Function inside Main
        let mut call_props = HashMap::new();
        call_props.insert("target_function".to_string(), json!("my_func"));
        call_props.insert("arguments".to_string(), json!([{"name": "x", "type": "i32"}]));
        
        let call_node = Node {
            id: "call".to_string(),
            plugin_type: "call-function".to_string(),
            label: Some("Call".to_string()),
            properties: call_props,
            parent_id: Some("main_container".to_string()),
        };
        
        // Connection Main Start -> Call
        let mut mapping = HashMap::new();
        mapping.insert("x".to_string(), "42".to_string());
        
        let conn2 = Connection {
            from: "main_start".to_string(),
            to: "call".to_string(),
            connection_type: ConnectionType::Simple,
            variable_mapping: Some(mapping),
        };
        
        let flow = Flow {
            nodes: vec![func_container, start_node, debug_node, main_container, main_start, call_node],
            connections: vec![conn1, conn2],
        };
        
        let code = generate_rust(&flow).unwrap();
        
        println!("{}", code);
        
        assert!(code.contains("async fn my_func(x: i32) {"));
        assert!(code.contains("println!(\"{:?}\", x);"));
        assert!(code.contains("#[tokio::main]"));
        assert!(code.contains("async fn main() {"));
        assert!(code.contains("my_func(42).await;"));
    }
}
