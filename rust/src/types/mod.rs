//! WDL type model.

/// High-level type family for traversal and validation dispatch.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TypeComponentType {
    Primitive,
    TypeRef,
    Array,
    Pair,
    Map,
}

/// Primitive WDL type names.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WdlPrimitiveKind {
    Boolean,
    Int,
    Float,
    String,
    File,
    Directory,
    Object,
}

impl WdlPrimitiveKind {
    /// Returns the canonical WDL source spelling of this primitive type.
    pub fn to_wdl_str(self) -> &'static str {
        match self {
            WdlPrimitiveKind::Boolean => "Boolean",
            WdlPrimitiveKind::Int => "Int",
            WdlPrimitiveKind::Float => "Float",
            WdlPrimitiveKind::String => "String",
            WdlPrimitiveKind::File => "File",
            WdlPrimitiveKind::Directory => "Directory",
            WdlPrimitiveKind::Object => "Object",
        }
    }
}

/// A primitive WDL type with optional marker.
#[derive(Debug, Clone, PartialEq)]
pub struct WdlPrimitiveType {
    pub primitive_kind: WdlPrimitiveKind,
    pub optional: bool,
}

impl WdlPrimitiveType {
    pub fn new(primitive_kind: WdlPrimitiveKind) -> Self {
        Self {
            primitive_kind,
            optional: false,
        }
    }

    pub fn optional(primitive_kind: WdlPrimitiveKind) -> Self {
        Self {
            primitive_kind,
            optional: true,
        }
    }
}

/// An `Array[T]` type, optionally non-empty (`+`) and/or optional (`?`).
#[derive(Debug, Clone, PartialEq)]
pub struct WdlArrayType {
    pub member_type: Box<WdlType>,
    pub non_empty: bool,
    pub optional: bool,
}

impl WdlArrayType {
    pub fn new(member_type: WdlType) -> Self {
        Self {
            member_type: Box::new(member_type),
            non_empty: false,
            optional: false,
        }
    }
}

/// A `Map[K, V]` type, optionally optional (`?`).
#[derive(Debug, Clone, PartialEq)]
pub struct WdlMapType {
    pub key_type: Box<WdlType>,
    pub value_type: Box<WdlType>,
    pub optional: bool,
}

impl WdlMapType {
    pub fn new(key_type: WdlType, value_type: WdlType) -> Self {
        Self {
            key_type: Box::new(key_type),
            value_type: Box::new(value_type),
            optional: false,
        }
    }
}

/// A `Pair[L, R]` type, optionally optional (`?`).
#[derive(Debug, Clone, PartialEq)]
pub struct WdlPairType {
    pub left_type: Box<WdlType>,
    pub right_type: Box<WdlType>,
    pub optional: bool,
}

impl WdlPairType {
    pub fn new(left_type: WdlType, right_type: WdlType) -> Self {
        Self {
            left_type: Box::new(left_type),
            right_type: Box::new(right_type),
            optional: false,
        }
    }
}

/// A user-defined type reference (struct or enum name), optionally optional (`?`).
#[derive(Debug, Clone, PartialEq)]
pub struct WdlTypeRefType {
    pub reference_name: String,
    pub optional: bool,
}

impl WdlTypeRefType {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            reference_name: name.into(),
            optional: false,
        }
    }
}

/// WDL type node — an enum over all type families.
#[derive(Debug, Clone, PartialEq)]
pub enum WdlType {
    Primitive(WdlPrimitiveType),
    Array(WdlArrayType),
    Map(Box<WdlMapType>),
    Pair(Box<WdlPairType>),
    TypeRef(WdlTypeRefType),
}

impl WdlType {
    /// Returns the broad category of this type node.
    pub fn component_type(&self) -> TypeComponentType {
        match self {
            WdlType::Primitive(_) => TypeComponentType::Primitive,
            WdlType::Array(_) => TypeComponentType::Array,
            WdlType::Map(_) => TypeComponentType::Map,
            WdlType::Pair(_) => TypeComponentType::Pair,
            WdlType::TypeRef(_) => TypeComponentType::TypeRef,
        }
    }

    /// Returns whether this type is marked optional (`?`).
    pub fn is_optional(&self) -> bool {
        match self {
            WdlType::Primitive(t) => t.optional,
            WdlType::Array(t) => t.optional,
            WdlType::Map(t) => t.optional,
            WdlType::Pair(t) => t.optional,
            WdlType::TypeRef(t) => t.optional,
        }
    }

    /// Sets the optional flag on this type, returning the modified type.
    pub fn with_optional(mut self, optional: bool) -> Self {
        match &mut self {
            WdlType::Primitive(t) => t.optional = optional,
            WdlType::Array(t) => t.optional = optional,
            WdlType::Map(t) => t.optional = optional,
            WdlType::Pair(t) => t.optional = optional,
            WdlType::TypeRef(t) => t.optional = optional,
        }
        self
    }
}
