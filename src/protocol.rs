//! The current latest sentry protocol version.

use chrono::{DateTime, Utc};
use uuid::Uuid;

use common::Values;
use meta::Annotated;
use utils::{annotated, serde_chrono};

fn default_breadcrumb_type() -> Annotated<String> {
    "default".to_string().into()
}

/// A breadcrumb.
#[derive(Debug, Deserialize, PartialEq, Serialize)]
pub struct Breadcrumb {
    /// The timestamp of the breadcrumb (required).
    #[serde(with = "serde_chrono")]
    pub timestamp: Annotated<DateTime<Utc>>,

    /// The type of the breadcrumb.
    #[serde(default = "default_breadcrumb_type", rename = "type")]
    pub ty: Annotated<String>,

    /// The optional category of the breadcrumb.
    #[serde(default, skip_serializing_if = "annotated::is_none")]
    pub category: Annotated<Option<String>>,
}

#[cfg(test)]
mod test_breadcrumb {
    use super::*;
    use serde_json;

    #[test]
    fn test_roundtrip() {
        let json = r#"{
  "timestamp": 42,
  "type": "mytype",
  "category": "mycategory"
}"#;

        let breadcrumb = Annotated::from(Breadcrumb {
            timestamp: serde_chrono::timestamp_to_datetime(42.0).into(),
            ty: "mytype".to_string().into(),
            category: Some("mycategory".to_string()).into(),
        });

        assert_eq!(breadcrumb, serde_json::from_str(json).unwrap());
        assert_eq!(json, &serde_json::to_string_pretty(&breadcrumb).unwrap());
    }

    #[test]
    fn test_default_values() {
        let input = r#"{"timestamp":42}"#;
        let output = r#"{"timestamp":42,"type":"default"}"#;

        let breadcrumb = Annotated::from(Breadcrumb {
            timestamp: serde_chrono::timestamp_to_datetime(42.0).into(),
            ty: default_breadcrumb_type(),
            category: None.into(),
        });

        assert_eq!(breadcrumb, serde_json::from_str(input).unwrap());
        assert_eq!(output, &serde_json::to_string(&breadcrumb).unwrap());
    }
}

/// Represents a full event for Sentry.
#[derive(Debug, Default, Deserialize, PartialEq, Serialize)]
pub struct Event {
    /// The unique identifier of this event.
    #[serde(default, rename = "event_id", skip_serializing_if = "annotated::is_none")]
    pub id: Annotated<Option<Uuid>>,

    /// List of breadcrumbs recorded before this event.
    #[serde(default, skip_serializing_if = "annotated::is_empty_values")]
    pub breadcrumbs: Annotated<Values<Annotated<Breadcrumb>>>,
}

#[cfg(test)]
mod test_event {
    use super::*;
    use serde_json;

    #[test]
    fn test_roundtrip() {
        // NOTE: Interfaces will be tested separately.
        let json = r#"{
  "event_id": "52df9022-8352-46ee-b317-dbd739ccd059"
}"#;

        let event = Annotated::from(Event {
            id: Some("52df9022-8352-46ee-b317-dbd739ccd059".parse().unwrap()).into(),
            breadcrumbs: Default::default(),
        });

        assert_eq!(event, serde_json::from_str(json).unwrap());
        assert_eq!(json, &serde_json::to_string_pretty(&event).unwrap());
    }

    #[test]
    fn test_default_values() {
        let json = r#"{"event_id":"52df9022-8352-46ee-b317-dbd739ccd059"}"#;
        let event = Annotated::from(Event {
            id: Some("52df9022-8352-46ee-b317-dbd739ccd059".parse().unwrap()).into(),
            breadcrumbs: Default::default(),
        });

        assert_eq!(event, serde_json::from_str(json).unwrap());
        assert_eq!(json, &serde_json::to_string(&event).unwrap());
    }
}
