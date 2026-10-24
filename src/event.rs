use crate::dispatch::DISPATCH;
use serde::Serialize;
use serde_json::{Map, Value};
use uuid::Uuid;

/// Tracks an event, such as `agent_run`, and sends it in the background like a log.
///
/// `name` matches `^[a-z][a-z0-9_]{0,63}$`.
/// `pageview`, `pageleave` and `browser_error` belong to the website tracker.
/// `props` holds up to 10 strings, numbers or booleans, never personal data.
/// `user_id` is the signed-in user's internal ID, never an email address.
/// An invalid name, or a user ID that contains `@`, drops the event with a warning.
///
/// ```no_run
/// logdash::create_logdash(logdash::Config::default().api_key("Your Domain Key".into()));
///
/// logdash::track(
///     "agent_run",
///     &[("agent", "support".into()), ("steps", 4.into()), ("success", true.into())],
///     Some("42"),
/// );
/// ```
pub fn track(name: &str, props: &[(&str, PropValue)], user_id: Option<&str>) {
    if !valid_name(name) {
        eprintln!(
            "Event {name:?} dropped: the name must match ^[a-z][a-z0-9_]{{0,63}}$ and not be pageview, pageleave or browser_error"
        );
        return;
    }
    if user_id.is_some_and(|id| id.contains('@')) {
        eprintln!(
            "Event {name:?} dropped: the user ID must be an internal ID, never an email address"
        );
        return;
    }
    let Some(dispatch) = DISPATCH.get() else {
        eprintln!("Event {name:?} dropped: call logdash::create_logdash first");
        return;
    };
    dispatch.dispatch_event(EventMessage {
        id: Uuid::new_v4().to_string(),
        name: name.into(),
        props: props
            .iter()
            .map(|(key, value)| (key.to_string(), value.0.clone()))
            .collect(),
        user_id: user_id.map(Into::into),
    });
}

fn valid_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    bytes.first().is_some_and(u8::is_ascii_lowercase)
        && bytes.len() <= 64
        && bytes
            .iter()
            .all(|&c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == b'_')
        && !matches!(name, "pageview" | "pageleave" | "browser_error")
}

/// The value of an event property: a string, a number or a boolean.
#[derive(Debug, Clone)]
pub struct PropValue(Value);

macro_rules! prop_value_from {
    ($($t:ty),*) => {$(
        impl From<$t> for PropValue {
            fn from(value: $t) -> Self {
                Self(value.into())
            }
        }
    )*};
}

prop_value_from!(
    &str, String, bool, i8, i16, i32, i64, isize, u8, u16, u32, u64, usize, f32, f64
);

/// `id` is a UUID v4, so a retried request stores the event once.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EventMessage {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Map::is_empty")]
    pub props: Map<String, Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::valid_name;

    #[test]
    fn event_names_follow_the_api_rule() {
        for name in ["a", "agent_run", "step2_done", "a".repeat(64).as_str()] {
            assert!(valid_name(name), "{name:?} is valid");
        }
        for name in [
            "",
            "Agent",
            "agent run",
            "agent-run",
            "2fa",
            "_run",
            "zażółć",
            "a".repeat(65).as_str(),
            "pageview",
            "pageleave",
            "browser_error",
        ] {
            assert!(!valid_name(name), "{name:?} is invalid");
        }
    }
}
