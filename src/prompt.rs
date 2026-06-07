use serde::Serialize;
use thiserror::Error;

/// Errors from compiling or rendering a [`Prompt`].
#[derive(Debug, Error)]
pub enum PromptError {
    /// Template syntax error.
    #[error("template syntax error: {0}")]
    Syntax(String),
    /// Render error (missing var, invalid op, etc.).
    #[error("render error: {0}")]
    Render(String),
}

impl From<minijinja::Error> for PromptError {
    fn from(e: minijinja::Error) -> Self {
        match e.kind() {
            minijinja::ErrorKind::SyntaxError => PromptError::Syntax(e.to_string()),
            _ => PromptError::Render(e.to_string()),
        }
    }
}

/// One compiled Jinja2 template.
#[derive(Clone)]
pub struct Prompt {
    env: minijinja::Environment<'static>,
    name: String,
}

impl Prompt {
    /// Compile `source` as a template. Validates Jinja2 syntax up front.
    pub fn new(source: impl Into<String>) -> Result<Self, PromptError> {
        let mut env = minijinja::Environment::new();
        // Fail hard on missing variables so bugs surface early instead of
        // silently rendering an empty string in place of the missing value.
        env.set_undefined_behavior(minijinja::UndefinedBehavior::Strict);
        env.add_template_owned("inline", source.into())?;
        Ok(Self {
            env,
            name: "inline".into(),
        })
    }

    /// Render against any `serde::Serialize` value (typically a `serde_json::Value`).
    pub fn render<T: Serialize>(&self, vars: &T) -> Result<String, PromptError> {
        let tmpl = self.env.get_template(&self.name)?;
        Ok(tmpl.render(vars)?)
    }
}
