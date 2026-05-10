# agentprompt

[![crates.io](https://img.shields.io/crates/v/agentprompt.svg)](https://crates.io/crates/agentprompt)
[![docs.rs](https://docs.rs/agentprompt/badge.svg)](https://docs.rs/agentprompt)
[![License: MIT](https://img.shields.io/badge/license-MIT-green.svg)](LICENSE)

LLM prompt templates with Jinja2 syntax. Render `system` / `user` / `assistant` turns into a typed message list ready for the Anthropic or OpenAI SDK.

```toml
[dependencies]
agentprompt = "0.1"
```

## Why

[pydantic-ai #921 (22 reactions)](https://github.com/pydantic/pydantic-ai/issues/921) is the open issue everyone keeps re-asking: how do I template my prompts cleanly? Jinja2 is the de-facto answer in Python (jxnl/instructor and many other Python libs ship with it). Rust LLM users get nothing. `agentprompt` is the smallest possible primitive: minijinja under the hood, role-aware on top.

## Quick start

```rust
use agentprompt::{Messages, Role};
use serde_json::json;

let messages = Messages::new()
    .system("You are a {{adjective}} assistant.")
    .user("Question: {{q}}")
    .render(&json!({"adjective": "concise", "q": "what is 2+2?"}))
    .unwrap();

// messages[0] = { role: System, content: "You are a concise assistant." }
// messages[1] = { role: User,   content: "Question: what is 2+2?" }
```

`Message` derives `Serialize` with a lowercase `role` field — drop straight into Anthropic's `messages` payload or OpenAI's `messages` array.

## With your provider SDK

Anthropic example (pseudo):

```rust,ignore
let messages = Messages::new()
    .system("You translate to {{lang}}.")
    .user("Translate: {{text}}")
    .render(&json!({"lang": "fr", "text": "hello"}))?;

let resp = anthropic_client
    .messages()
    .create()
    .model("claude-sonnet-4-20250514")
    .max_tokens(200)
    .messages(serde_json::to_value(&messages)?)  // already shaped right
    .send().await?;
```

## Single-template form

If you don't need role splitting, use `Prompt` directly:

```rust
use agentprompt::Prompt;
use serde_json::json;

let p = Prompt::new("{% if items %}You have {{items|length}} todos.{% else %}Nothing pending.{% endif %}").unwrap();
let out = p.render(&json!({"items": ["A", "B"]})).unwrap();
assert_eq!(out, "You have 2 todos.");
```

## Strictness

Missing vars **fail loud** (strict mode is on). We'd rather break tests than silently send the model a prompt with `{{q}}` literal in it.

## What it doesn't do

- Doesn't include / `{% include %}` other templates from disk in v0.1 (you can use `Prompt::new(file_contents)` after reading them yourself).
- Doesn't autoescape (these aren't HTML).
- Doesn't hand-craft per-provider message shapes — it gives you `Vec<Message>`; you map to the SDK's type.

## License

MIT
