//! Template management for the agents core system
//!
//! This module provides template registry, loading, and processing functionality.

use super::config::{PromptTemplate, TemplateType, TemplateVariable, VariableType};
use super::error::{AgentError, Result};
use handlebars::Handlebars;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

/// Template registry for managing prompt templates
pub struct TemplateRegistry {
    templates: HashMap<String, PromptTemplate>,
    handlebars: Handlebars<'static>,
}

/// Template processing context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TemplateContext {
    /// Variables and their values
    pub variables: HashMap<String, serde_json::Value>,
    /// Metadata about the template processing
    pub metadata: HashMap<String, String>,
}

impl TemplateRegistry {
    /// Create a new template registry
    pub fn new() -> Result<Self> {
        let mut handlebars = Handlebars::new();

        // Configure handlebars
        handlebars.set_strict_mode(true);
        handlebars.register_escape_fn(handlebars::no_escape);

        Ok(Self {
            templates: HashMap::new(),
            handlebars,
        })
    }

    /// Register a new template
    pub fn register_template(&mut self, template: PromptTemplate) -> Result<()> {
        // Validate template syntax
        if let Err(error) = self
            .handlebars
            .register_template_string(&template.name, &template.content)
        {
            return Err(AgentError::Template(format!(
                "Invalid template syntax: {}",
                error
            )));
        }

        self.templates.insert(template.name.clone(), template);
        Ok(())
    }

    /// Get a template by name
    pub fn get_template(&self, name: &str) -> Option<&PromptTemplate> {
        self.templates.get(name)
    }

    /// List all registered templates
    pub fn list_templates(&self) -> Vec<&PromptTemplate> {
        self.templates.values().collect()
    }

    /// List templates by type
    pub fn list_templates_by_type(&self, template_type: TemplateType) -> Vec<&PromptTemplate> {
        self.templates
            .values()
            .filter(|t| t.r#type == template_type)
            .collect()
    }

    /// Render a template with context
    pub fn render_template(
        &self,
        template_name: &str,
        context: &TemplateContext,
    ) -> Result<String> {
        let template = self.templates.get(template_name).ok_or_else(|| {
            AgentError::Template(format!("Template not found: {}", template_name))
        })?;

        // Validate required variables
        self.validate_variables(template, context)?;

        // Render template
        self.handlebars
            .render(template_name, context)
            .map_err(|e| AgentError::Template(format!("Failed to render template: {}", e)))
    }

    /// Validate that all required variables are provided
    fn validate_variables(
        &self,
        template: &PromptTemplate,
        context: &TemplateContext,
    ) -> Result<()> {
        for variable in &template.variables {
            if variable.required && !context.variables.contains_key(&variable.name) {
                return Err(AgentError::Template(format!(
                    "Required variable '{}' not provided",
                    variable.name
                )));
            }
        }
        Ok(())
    }

    /// Load templates from a directory
    pub fn load_templates_from_directory<P: AsRef<Path>>(&mut self, dir: P) -> Result<usize> {
        let dir = dir.as_ref();
        if !dir.exists() {
            return Ok(0);
        }

        let mut loaded_count = 0;

        for entry in std::fs::read_dir(dir).map_err(|e| {
            AgentError::Storage(format!("Failed to read templates directory: {}", e))
        })? {
            let entry = entry.map_err(|e| {
                AgentError::Storage(format!("Failed to read directory entry: {}", e))
            })?;
            let path = entry.path();

            if path.is_file()
                && path.extension().and_then(|s| s.to_str()) == Some("json")
                && let Ok(template) = self.load_template_from_file(&path)
            {
                self.register_template(template)?;
                loaded_count += 1;
            }
        }
        Ok(loaded_count)
    }

    /// Load a single template from a file
    pub fn load_template_from_file<P: AsRef<Path>>(&self, file_path: P) -> Result<PromptTemplate> {
        let content = std::fs::read_to_string(file_path)
            .map_err(|e| AgentError::Storage(format!("Failed to read template file: {}", e)))?;

        serde_json::from_str(&content)
            .map_err(|e| AgentError::Template(format!("Failed to parse template file: {}", e)))
    }

    /// Save a template to a file
    pub fn save_template_to_file<P: AsRef<Path>>(
        &self,
        template: &PromptTemplate,
        file_path: P,
    ) -> Result<()> {
        let content = serde_json::to_string_pretty(template)
            .map_err(|e| AgentError::Template(format!("Failed to serialize template: {}", e)))?;

        // Ensure parent directory exists
        if let Some(parent) = file_path.as_ref().parent() {
            std::fs::create_dir_all(parent).map_err(|e| {
                AgentError::Storage(format!("Failed to create template directory: {}", e))
            })?;
        }

        std::fs::write(file_path, content)
            .map_err(|e| AgentError::Storage(format!("Failed to write template file: {}", e)))
    }

    /// Remove a template
    pub fn remove_template(&mut self, name: &str) -> Option<PromptTemplate> {
        self.handlebars.unregister_template(name);
        self.templates.remove(name)
    }

    /// Create a template context from variables
    pub fn create_context(variables: HashMap<String, serde_json::Value>) -> TemplateContext {
        TemplateContext {
            variables,
            metadata: HashMap::new(),
        }
    }

    /// Add metadata to context
    pub fn add_metadata(context: &mut TemplateContext, key: String, value: String) {
        context.metadata.insert(key, value);
    }

    /// Get default templates for each type
    pub fn get_default_templates() -> Vec<PromptTemplate> {
        vec![
            // Memory template
            PromptTemplate {
                name: "memory-default".to_string(),
                description: "Default memory prompt template".to_string(),
                version: "1.0.0".to_string(),
                r#type: TemplateType::Memory,
                content: r#"# {{title}}

## Description
{{description}}

## Context
{{context}}

## Instructions
{{instructions}}

## Examples
{{examples}}
"#
                .to_string(),
                metadata: {
                    let mut m = HashMap::new();
                    m.insert("author".to_string(), "system".to_string());
                    m.insert("created".to_string(), "2024-01-01".to_string());
                    m
                },
                variables: vec![
                    TemplateVariable {
                        name: "title".to_string(),
                        description: "Title of the memory prompt".to_string(),
                        r#type: VariableType::String,
                        required: true,
                        default: Some("Memory Prompt".to_string()),
                        values: None,
                    },
                    TemplateVariable {
                        name: "description".to_string(),
                        description: "Description of what this prompt does".to_string(),
                        r#type: VariableType::String,
                        required: true,
                        default: None,
                        values: None,
                    },
                    TemplateVariable {
                        name: "context".to_string(),
                        description: "Context information for the prompt".to_string(),
                        r#type: VariableType::String,
                        required: false,
                        default: None,
                        values: None,
                    },
                    TemplateVariable {
                        name: "instructions".to_string(),
                        description: "Specific instructions for the AI".to_string(),
                        r#type: VariableType::String,
                        required: true,
                        default: None,
                        values: None,
                    },
                    TemplateVariable {
                        name: "examples".to_string(),
                        description: "Example inputs and outputs".to_string(),
                        r#type: VariableType::String,
                        required: false,
                        default: None,
                        values: None,
                    },
                ],
            },
            // Sub-agent template
            PromptTemplate {
                name: "subagent-default".to_string(),
                description: "Default sub-agent prompt template".to_string(),
                version: "1.0.0".to_string(),
                r#type: TemplateType::SubAgent,
                content: r#"# {{agent_name}} Agent

## Role
{{role}}

## Capabilities
{{capabilities}}

## Instructions
{{instructions}}

## Constraints
{{constraints}}

## Tools Available
{{tools}}

## Examples
{{examples}}
"#
                .to_string(),
                metadata: {
                    let mut m = HashMap::new();
                    m.insert("author".to_string(), "system".to_string());
                    m.insert("created".to_string(), "2024-01-01".to_string());
                    m
                },
                variables: vec![
                    TemplateVariable {
                        name: "agent_name".to_string(),
                        description: "Name of the sub-agent".to_string(),
                        r#type: VariableType::String,
                        required: true,
                        default: None,
                        values: None,
                    },
                    TemplateVariable {
                        name: "role".to_string(),
                        description: "Role and purpose of the agent".to_string(),
                        r#type: VariableType::String,
                        required: true,
                        default: None,
                        values: None,
                    },
                    TemplateVariable {
                        name: "capabilities".to_string(),
                        description: "What the agent can do".to_string(),
                        r#type: VariableType::String,
                        required: true,
                        default: None,
                        values: None,
                    },
                    TemplateVariable {
                        name: "instructions".to_string(),
                        description: "Specific instructions for the agent".to_string(),
                        r#type: VariableType::String,
                        required: true,
                        default: None,
                        values: None,
                    },
                    TemplateVariable {
                        name: "constraints".to_string(),
                        description: "Limitations and constraints".to_string(),
                        r#type: VariableType::String,
                        required: false,
                        default: None,
                        values: None,
                    },
                    TemplateVariable {
                        name: "tools".to_string(),
                        description: "Available tools for the agent".to_string(),
                        r#type: VariableType::String,
                        required: false,
                        default: None,
                        values: None,
                    },
                    TemplateVariable {
                        name: "examples".to_string(),
                        description: "Example usage scenarios".to_string(),
                        r#type: VariableType::String,
                        required: false,
                        default: None,
                        values: None,
                    },
                ],
            },
            // Command template
            PromptTemplate {
                name: "command-default".to_string(),
                description: "Default command prompt template".to_string(),
                version: "1.0.0".to_string(),
                r#type: TemplateType::Command,
                content: r#"# {{command_name}}

## Description
{{description}}

## Usage
```
{{usage}}
```

## Arguments
{{arguments}}

## Examples
{{examples}}

## Notes
{{notes}}
"#
                .to_string(),
                metadata: {
                    let mut m = HashMap::new();
                    m.insert("author".to_string(), "system".to_string());
                    m.insert("created".to_string(), "2024-01-01".to_string());
                    m
                },
                variables: vec![
                    TemplateVariable {
                        name: "command_name".to_string(),
                        description: "Name of the command".to_string(),
                        r#type: VariableType::String,
                        required: true,
                        default: None,
                        values: None,
                    },
                    TemplateVariable {
                        name: "description".to_string(),
                        description: "What the command does".to_string(),
                        r#type: VariableType::String,
                        required: true,
                        default: None,
                        values: None,
                    },
                    TemplateVariable {
                        name: "usage".to_string(),
                        description: "Command usage syntax".to_string(),
                        r#type: VariableType::String,
                        required: true,
                        default: None,
                        values: None,
                    },
                    TemplateVariable {
                        name: "arguments".to_string(),
                        description: "Available arguments and options".to_string(),
                        r#type: VariableType::String,
                        required: false,
                        default: None,
                        values: None,
                    },
                    TemplateVariable {
                        name: "examples".to_string(),
                        description: "Usage examples".to_string(),
                        r#type: VariableType::String,
                        required: false,
                        default: None,
                        values: None,
                    },
                    TemplateVariable {
                        name: "notes".to_string(),
                        description: "Additional notes and warnings".to_string(),
                        r#type: VariableType::String,
                        required: false,
                        default: None,
                        values: None,
                    },
                ],
            },
        ]
    }
}

impl Default for TemplateRegistry {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            // Create a minimal registry if initialization fails
            let handlebars = Handlebars::new();
            Self {
                templates: HashMap::new(),
                handlebars,
            }
        })
    }
}
