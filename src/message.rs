use crate::prompt::{Prompt, PromptError};
use serde::{Deserialize, Serialize};

/// Standard chat roles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    /// System / instructions.
    System,
    /// End-user message.
    User,
    /// Model / assistant reply.
    Assistant,
}

impl Role {
    /// Render the role as the lowercase string most provider APIs expect.
    pub fn as_str(&self) -> &'static str {
        match self {
            Role::System => "system",
            Role::User => "user",
            Role::Assistant => "assistant",
        }
    }
}

/// One rendered message.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Message {
    /// `system`, `user`, or `assistant`.
    pub role: Role,
    /// Plain text content.
    pub content: String,
}

/// Ordered list of (role, template) entries that renders into `Vec<Message>`.
#[derive(Default, Clone)]
pub struct Messages {
    entries: Vec<(Role, String)>,
}

impl Messages {
    /// Empty list.
    pub fn new() -> Self {
        Self::default()
    }

    /// Append a system-role template.
    pub fn system(mut self, tmpl: impl Into<String>) -> Self {
        self.entries.push((Role::System, tmpl.into()));
        self
    }

    /// Append a user-role template.
    pub fn user(mut self, tmpl: impl Into<String>) -> Self {
        self.entries.push((Role::User, tmpl.into()));
        self
    }

    /// Append an assistant-role template.
    pub fn assistant(mut self, tmpl: impl Into<String>) -> Self {
        self.entries.push((Role::Assistant, tmpl.into()));
        self
    }

    /// Append a template under an arbitrary role.
    pub fn push(mut self, role: Role, tmpl: impl Into<String>) -> Self {
        self.entries.push((role, tmpl.into()));
        self
    }

    /// Render every entry against `vars`. Each template is compiled fresh;
    /// for hot loops, compile once via [`Prompt`] and reuse.
    pub fn render<T: serde::Serialize>(&self, vars: &T) -> Result<Vec<Message>, PromptError> {
        let mut out = Vec::with_capacity(self.entries.len());
        for (role, tmpl) in &self.entries {
            let p = Prompt::new(tmpl.clone())?;
            out.push(Message {
                role: *role,
                content: p.render(vars)?,
            });
        }
        Ok(out)
    }

    /// Number of templates queued.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// True if no templates are queued.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}
