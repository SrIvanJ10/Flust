use std::collections::HashMap;

/// Simple template engine for replacing variables in Rust templates
/// Follows Single Responsibility Principle - only handles template processing
pub struct TemplateEngine;

impl TemplateEngine {
    /// Replace template variables with actual values
    /// Supports both {{variable}} and {{#if condition}}...{{/if}} syntax
    pub fn render(template: &str, context: &HashMap<String, String>) -> String {
        let mut result = template.to_string();
        
        // Handle each loops first: {{#each array}}...{{/each}}
        result = Self::process_each_loops(result, context);
        
        // Replace simple variables {{variable_name}}
        for (key, value) in context {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }
        
        // Handle conditional blocks: {{#if key}}...{{else}}...{{/if}}
        result = Self::process_conditionals(result, context);
        
        result
    }
    
    fn process_each_loops(template: String, context: &HashMap<String, String>) -> String {
        let mut result = template;
        
        loop {
            if let Some(each_start) = result.find("{{#each ") {
                if let Some(each_end) = result[each_start..].find("}}") {
                    let condition_end = each_start + each_end;
                    let array_name = result[each_start + 8..condition_end].trim();
                    
                    // Find the closing {{/each}}
                    if let Some(endeach_pos) = result[condition_end..].find("{{/each}}") {
                        let endeach_start = condition_end + endeach_pos;
                        let block_content = &result[condition_end + 2..endeach_start];
                        
                        // Try to parse the array from context
                        let replacement = if let Some(array_json) = context.get(array_name) {
                            if let Ok(array) = serde_json::from_str::<Vec<serde_json::Value>>(array_json) {
                                let mut output = String::new();
                                for (index, item) in array.iter().enumerate() {
                                    let mut item_output = block_content.to_string();
                                    
                                    // Replace {{@last}} with true/false
                                    let is_last = index == array.len() - 1;
                                    
                                    // Handle {{#unless @last}}...{{/unless}}
                                    while let Some(unless_start) = item_output.find("{{#unless @last}}") {
                                        if let Some(unless_end_rel) = item_output[unless_start..].find("{{/unless}}") {
                                            let unless_end = unless_start + unless_end_rel;
                                            let full_end = unless_end + 11; // length of {{/unless}}
                                            
                                            if is_last {
                                                // Remove the whole block
                                                item_output.replace_range(unless_start..full_end, "");
                                            } else {
                                                // Remove just the tags, keep content
                                                let content_start = unless_start + 17; // length of {{#unless @last}}
                                                let content = item_output[content_start..unless_end].to_string();
                                                item_output.replace_range(unless_start..full_end, &content);
                                            }
                                        } else {
                                            break;
                                        }
                                    }
                                    
                                    // Replace object properties like {{name}}, {{type}}
                                    if let Some(obj) = item.as_object() {
                                        for (key, value) in obj {
                                            let placeholder = format!("{{{{{}}}}}", key);
                                            let value_str = match value {
                                                serde_json::Value::String(s) => s.clone(),
                                                serde_json::Value::Number(n) => n.to_string(),
                                                serde_json::Value::Bool(b) => b.to_string(),
                                                _ => value.to_string(),
                                            };
                                            item_output = item_output.replace(&placeholder, &value_str);
                                        }
                                    }
                                    
                                    output.push_str(&item_output);
                                }
                                output
                            } else {
                                String::new()
                            }
                        } else {
                            String::new()
                        };
                        
                        // Replace the entire each block
                        let before = &result[..each_start];
                        let after = &result[endeach_start + 9..];
                        result = format!("{}{}{}", before, replacement, after);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        result
    }
    
    fn process_conditionals(template: String, context: &HashMap<String, String>) -> String {
        let mut result = template;
        
        // Simple regex-like processing for {{#if key}}...{{/if}}
        loop {
            if let Some(if_start) = result.find("{{#if ") {
                if let Some(if_end) = result[if_start..].find("}}") {
                    let condition_end = if_start + if_end;
                    let condition = result[if_start + 6..condition_end].trim();
                    
                    // Find the closing {{/if}}
                    if let Some(endif_pos) = result[condition_end..].find("{{/if}}") {
                        let endif_start = condition_end + endif_pos;
                        let block_content = &result[condition_end + 2..endif_start];
                        
                        // Check for {{else}}
                        let (true_block, false_block) = if let Some(else_pos) = block_content.find("{{else}}") {
                            (
                                &block_content[..else_pos],
                                &block_content[else_pos + 8..]
                            )
                        } else {
                            (block_content, "")
                        };
                        
                        // Evaluate condition - true if variable exists and is non-empty
                        let condition_met = context.get(condition)
                            .map(|v| !v.is_empty())
                            .unwrap_or(false);
                        
                        let replacement = if condition_met {
                            true_block.trim()
                        } else {
                            false_block.trim()
                        };
                        
                        // Replace the entire conditional block
                        let before = &result[..if_start];
                        let after = &result[endif_start + 7..];
                        result = format!("{}{}{}", before, replacement, after);
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
        
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_replacement() {
        let template = "Hello {{name}}!";
        let mut context = HashMap::new();
        context.insert("name".to_string(), "World".to_string());
        
        let result = TemplateEngine::render(template, &context);
        assert_eq!(result, "Hello World!");
    }
    
    #[test]
    fn test_conditional_true() {
        let template = "{{#if label}}Label: {{label}}{{else}}No label{{/if}}";
        let mut context = HashMap::new();
        context.insert("label".to_string(), "Test".to_string());
        
        let result = TemplateEngine::render(template, &context);
        assert_eq!(result, "Label: Test");
    }
    
    #[test]
    fn test_conditional_false() {
        let template = "{{#if label}}Label: {{label}}{{else}}No label{{/if}}";
        let context = HashMap::new();
        
        let result = TemplateEngine::render(template, &context);
        assert_eq!(result, "No label");
    }
}
