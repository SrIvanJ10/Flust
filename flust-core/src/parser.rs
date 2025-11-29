use crate::ir::Flow;
use anyhow::Result;
use std::path::Path;
use std::fs;

pub fn parse_file<P: AsRef<Path>>(path: P) -> Result<Flow> {
    let content = fs::read_to_string(path)?;
    let flow: Flow = serde_yaml::from_str(&content)?;
    Ok(flow)
}

pub fn parse_str(content: &str) -> Result<Flow> {
    let flow: Flow = serde_yaml::from_str(content)?;
    Ok(flow)
}
