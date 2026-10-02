//! Base traits and generic building blocks for the WDL model.

use crate::expressions::WdlExpression;

/// Marker trait for every node in the WDL object model.
pub trait WdlNode: std::fmt::Debug {}

/// Generic key-value pair used throughout the WDL model (metadata entries,
/// runtime entries, map literal entries, call input bindings, etc.).
#[derive(Debug, Clone, PartialEq)]
pub struct WdlKeyValue<K, V> {
    pub key: K,
    pub value: Option<V>,
}

impl<K, V> WdlKeyValue<K, V> {
    pub fn new(key: K) -> Self {
        Self { key, value: None }
    }

    pub fn with_value(key: K, value: V) -> Self {
        Self {
            key,
            value: Some(value),
        }
    }
}

/// Key-value node whose key is a `String` and value is a `WdlExpression`.
pub type WdlStringKeyValue = WdlKeyValue<String, WdlExpression>;

/// Key-value node whose key and value are both `WdlExpression`.
pub type WdlExpressionKeyValue = WdlKeyValue<WdlExpression, WdlExpression>;
