//! Jinja2-syntax LLM prompt templates.
//!
//! Build chat-style prompts from Jinja2 templates per role. `Prompt` is a
//! single template; `Messages` is an ordered list of `(role, template)`
//! entries that renders into a typed `Vec<Message>` ready to feed an
//! Anthropic or OpenAI SDK.
//!
//! # Quick start
//!
//! ```
//! use agentprompt::{Messages, Role};
//! use serde_json::json;
//!
//! let messages = Messages::new()
//!     .system("You are a {{adjective}} assistant.")
//!     .user("Question: {{q}}")
//!     .render(&json!({"adjective": "concise", "q": "what is 2+2?"}))
//!     .unwrap();
//!
//! assert_eq!(messages.len(), 2);
//! assert_eq!(messages[0].role, Role::System);
//! assert_eq!(messages[0].content, "You are a concise assistant.");
//! assert_eq!(messages[1].content, "Question: what is 2+2?");
//! ```
#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

mod message;
mod prompt;

pub use crate::message::{Message, Messages, Role};
pub use crate::prompt::{Prompt, PromptError};
