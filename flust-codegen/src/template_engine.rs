use std::collections::HashMap;

/// Simple template engine for replacing variables in Rust templates
/// Follows Single Responsibility Principle - only handles template processing
pub struct TemplateEngine;

impl TemplateEngine {
    /// Replace template variables with actual values
    /// Supports both {{variable}} and {{#if condition}}...{{/if}} syntax
    pub fn render(template: &str, context: &HashMap<String, String>) -> String {
        let mut result = template.to_string();
        
        // Replace simple variables {{variable_name}}
        for (key, value) in context {
            let placeholder = format!("{{{{{}}}}}", key);
            result = result.replace(&placeholder, value);
        }
        
        // Handle conditional blocks: {{#if key}}...{{else}}...{{/if}}
        result = Self::process_conditionals(result, context);
        
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
