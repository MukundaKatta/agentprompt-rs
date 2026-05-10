use agentprompt::{Messages, Prompt, PromptError, Role};
use serde_json::json;

#[test]
fn basic_substitution() {
    let p = Prompt::new("Hello, {{name}}!").unwrap();
    assert_eq!(p.render(&json!({"name": "world"})).unwrap(), "Hello, world!");
}

#[test]
fn conditionals() {
    let p = Prompt::new(
        "{% if premium %}Welcome back!{% else %}Hi there{% endif %}",
    )
    .unwrap();
    assert_eq!(p.render(&json!({"premium": true})).unwrap(), "Welcome back!");
    assert_eq!(p.render(&json!({"premium": false})).unwrap(), "Hi there");
}

#[test]
fn loops() {
    let p = Prompt::new("{% for item in items %}- {{item}}\n{% endfor %}").unwrap();
    let out = p.render(&json!({"items": ["a", "b", "c"]})).unwrap();
    assert_eq!(out, "- a\n- b\n- c\n");
}

#[test]
fn missing_variable_fails_strict() {
    let p = Prompt::new("Hello, {{missing}}!").unwrap();
    let err = p.render(&json!({})).unwrap_err();
    assert!(matches!(err, PromptError::Render(_)));
}

#[test]
fn syntax_error_at_compile() {
    // `Prompt` doesn't impl Debug (minijinja::Environment doesn't), so we can't
    // use unwrap_err(); pattern-match on the Result instead.
    match Prompt::new("Hello {{") {
        Ok(_) => panic!("expected Syntax error"),
        Err(PromptError::Syntax(_)) => {}
        Err(other) => panic!("expected Syntax, got {other:?}"),
    }
}

#[test]
fn messages_renders_two_turn_chat() {
    let msgs = Messages::new()
        .system("You are a {{adjective}} assistant.")
        .user("Question: {{q}}")
        .render(&json!({"adjective": "concise", "q": "what is 2+2?"}))
        .unwrap();
    assert_eq!(msgs.len(), 2);
    assert_eq!(msgs[0].role, Role::System);
    assert_eq!(msgs[0].content, "You are a concise assistant.");
    assert_eq!(msgs[1].role, Role::User);
    assert_eq!(msgs[1].content, "Question: what is 2+2?");
}

#[test]
fn messages_supports_assistant_role() {
    let msgs = Messages::new()
        .user("Q1")
        .assistant("A1: {{prior}}")
        .user("Q2")
        .render(&json!({"prior": "context"}))
        .unwrap();
    assert_eq!(msgs.len(), 3);
    assert_eq!(msgs[1].role, Role::Assistant);
    assert_eq!(msgs[1].content, "A1: context");
}

#[test]
fn role_as_str() {
    assert_eq!(Role::System.as_str(), "system");
    assert_eq!(Role::User.as_str(), "user");
    assert_eq!(Role::Assistant.as_str(), "assistant");
}

#[test]
fn message_serializes_with_lowercase_role() {
    let m = agentprompt::Message {
        role: Role::User,
        content: "hi".into(),
    };
    let s = serde_json::to_string(&m).unwrap();
    assert!(s.contains("\"role\":\"user\""));
}

#[test]
fn empty_messages_renders_empty_vec() {
    let msgs = Messages::new().render(&json!({})).unwrap();
    assert!(msgs.is_empty());
}
