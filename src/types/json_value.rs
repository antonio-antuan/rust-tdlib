use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Represents a JSON value
pub trait TDJsonValue: Debug + RObject {}

/// Represents a JSON value
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum JsonValue {
    #[doc(hidden)]
    _Default,
    /// Returns application config, provided by the server. Can be called before authorization
    #[serde(rename = "getApplicationConfig")]
    GetApplicationConfig(GetApplicationConfig),
    /// Converts a JSON-serialized string to corresponding JsonValue object. Can be called synchronously
    #[serde(rename = "getJsonValue")]
    GetJsonValue(GetJsonValue),
    /// Represents a JSON array
    #[serde(rename = "jsonValueArray")]
    Array(JsonValueArray),
    /// Represents a boolean JSON value
    #[serde(rename = "jsonValueBoolean")]
    Boolean(JsonValueBoolean),
    /// Represents a null JSON value
    #[serde(rename = "jsonValueNull")]
    Null(JsonValueNull),
    /// Represents a numeric JSON value
    #[serde(rename = "jsonValueNumber")]
    Number(JsonValueNumber),
    /// Represents a JSON object
    #[serde(rename = "jsonValueObject")]
    Object(JsonValueObject),
    /// Represents a string JSON value
    #[serde(rename = "jsonValueString")]
    String(JsonValueString),
}

impl Default for JsonValue {
    fn default() -> Self {
        JsonValue::_Default
    }
}

impl RObject for JsonValue {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            JsonValue::GetApplicationConfig(t) => t.extra(),
            JsonValue::GetJsonValue(t) => t.extra(),
            JsonValue::Array(t) => t.extra(),
            JsonValue::Boolean(t) => t.extra(),
            JsonValue::Null(t) => t.extra(),
            JsonValue::Number(t) => t.extra(),
            JsonValue::Object(t) => t.extra(),
            JsonValue::String(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            JsonValue::GetApplicationConfig(t) => t.client_id(),
            JsonValue::GetJsonValue(t) => t.client_id(),
            JsonValue::Array(t) => t.client_id(),
            JsonValue::Boolean(t) => t.client_id(),
            JsonValue::Null(t) => t.client_id(),
            JsonValue::Number(t) => t.client_id(),
            JsonValue::Object(t) => t.client_id(),
            JsonValue::String(t) => t.client_id(),

            _ => None,
        }
    }
}

impl JsonValue {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, JsonValue::_Default)
    }
}

impl AsRef<JsonValue> for JsonValue {
    fn as_ref(&self) -> &JsonValue {
        self
    }
}

/// Represents a JSON array
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JsonValueArray {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The list of array elements

    #[serde(default)]
    values: Vec<JsonValue>,
}

impl RObject for JsonValueArray {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDJsonValue for JsonValueArray {}

impl JsonValueArray {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> JsonValueArrayBuilder {
        let mut inner = JsonValueArray::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        JsonValueArrayBuilder { inner }
    }

    pub fn values(&self) -> &Vec<JsonValue> {
        &self.values
    }
}

#[doc(hidden)]
pub struct JsonValueArrayBuilder {
    inner: JsonValueArray,
}

#[deprecated]
pub type RTDJsonValueArrayBuilder = JsonValueArrayBuilder;

impl JsonValueArrayBuilder {
    pub fn build(&self) -> JsonValueArray {
        self.inner.clone()
    }

    pub fn values(&mut self, values: Vec<JsonValue>) -> &mut Self {
        self.inner.values = values;
        self
    }
}

impl AsRef<JsonValueArray> for JsonValueArray {
    fn as_ref(&self) -> &JsonValueArray {
        self
    }
}

impl AsRef<JsonValueArray> for JsonValueArrayBuilder {
    fn as_ref(&self) -> &JsonValueArray {
        &self.inner
    }
}

/// Represents a boolean JSON value
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JsonValueBoolean {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The value

    #[serde(default)]
    value: bool,
}

impl RObject for JsonValueBoolean {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDJsonValue for JsonValueBoolean {}

impl JsonValueBoolean {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> JsonValueBooleanBuilder {
        let mut inner = JsonValueBoolean::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        JsonValueBooleanBuilder { inner }
    }

    pub fn value(&self) -> bool {
        self.value
    }
}

#[doc(hidden)]
pub struct JsonValueBooleanBuilder {
    inner: JsonValueBoolean,
}

#[deprecated]
pub type RTDJsonValueBooleanBuilder = JsonValueBooleanBuilder;

impl JsonValueBooleanBuilder {
    pub fn build(&self) -> JsonValueBoolean {
        self.inner.clone()
    }

    pub fn value(&mut self, value: bool) -> &mut Self {
        self.inner.value = value;
        self
    }
}

impl AsRef<JsonValueBoolean> for JsonValueBoolean {
    fn as_ref(&self) -> &JsonValueBoolean {
        self
    }
}

impl AsRef<JsonValueBoolean> for JsonValueBooleanBuilder {
    fn as_ref(&self) -> &JsonValueBoolean {
        &self.inner
    }
}

/// Represents a null JSON value
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JsonValueNull {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
}

impl RObject for JsonValueNull {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDJsonValue for JsonValueNull {}

impl JsonValueNull {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> JsonValueNullBuilder {
        let mut inner = JsonValueNull::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        JsonValueNullBuilder { inner }
    }
}

#[doc(hidden)]
pub struct JsonValueNullBuilder {
    inner: JsonValueNull,
}

#[deprecated]
pub type RTDJsonValueNullBuilder = JsonValueNullBuilder;

impl JsonValueNullBuilder {
    pub fn build(&self) -> JsonValueNull {
        self.inner.clone()
    }
}

impl AsRef<JsonValueNull> for JsonValueNull {
    fn as_ref(&self) -> &JsonValueNull {
        self
    }
}

impl AsRef<JsonValueNull> for JsonValueNullBuilder {
    fn as_ref(&self) -> &JsonValueNull {
        &self.inner
    }
}

/// Represents a numeric JSON value
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JsonValueNumber {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The value

    #[serde(default)]
    value: f32,
}

impl RObject for JsonValueNumber {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDJsonValue for JsonValueNumber {}

impl JsonValueNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> JsonValueNumberBuilder {
        let mut inner = JsonValueNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        JsonValueNumberBuilder { inner }
    }

    pub fn value(&self) -> f32 {
        self.value
    }
}

#[doc(hidden)]
pub struct JsonValueNumberBuilder {
    inner: JsonValueNumber,
}

#[deprecated]
pub type RTDJsonValueNumberBuilder = JsonValueNumberBuilder;

impl JsonValueNumberBuilder {
    pub fn build(&self) -> JsonValueNumber {
        self.inner.clone()
    }

    pub fn value(&mut self, value: f32) -> &mut Self {
        self.inner.value = value;
        self
    }
}

impl AsRef<JsonValueNumber> for JsonValueNumber {
    fn as_ref(&self) -> &JsonValueNumber {
        self
    }
}

impl AsRef<JsonValueNumber> for JsonValueNumberBuilder {
    fn as_ref(&self) -> &JsonValueNumber {
        &self.inner
    }
}

/// Represents a JSON object
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JsonValueObject {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The list of object members

    #[serde(default)]
    members: Vec<JsonObjectMember>,
}

impl RObject for JsonValueObject {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDJsonValue for JsonValueObject {}

impl JsonValueObject {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> JsonValueObjectBuilder {
        let mut inner = JsonValueObject::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        JsonValueObjectBuilder { inner }
    }

    pub fn members(&self) -> &Vec<JsonObjectMember> {
        &self.members
    }
}

#[doc(hidden)]
pub struct JsonValueObjectBuilder {
    inner: JsonValueObject,
}

#[deprecated]
pub type RTDJsonValueObjectBuilder = JsonValueObjectBuilder;

impl JsonValueObjectBuilder {
    pub fn build(&self) -> JsonValueObject {
        self.inner.clone()
    }

    pub fn members(&mut self, members: Vec<JsonObjectMember>) -> &mut Self {
        self.inner.members = members;
        self
    }
}

impl AsRef<JsonValueObject> for JsonValueObject {
    fn as_ref(&self) -> &JsonValueObject {
        self
    }
}

impl AsRef<JsonValueObject> for JsonValueObjectBuilder {
    fn as_ref(&self) -> &JsonValueObject {
        &self.inner
    }
}

/// Represents a string JSON value
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JsonValueString {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The value

    #[serde(default)]
    value: String,
}

impl RObject for JsonValueString {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDJsonValue for JsonValueString {}

impl JsonValueString {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> JsonValueStringBuilder {
        let mut inner = JsonValueString::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        JsonValueStringBuilder { inner }
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}

#[doc(hidden)]
pub struct JsonValueStringBuilder {
    inner: JsonValueString,
}

#[deprecated]
pub type RTDJsonValueStringBuilder = JsonValueStringBuilder;

impl JsonValueStringBuilder {
    pub fn build(&self) -> JsonValueString {
        self.inner.clone()
    }

    pub fn value<T: AsRef<str>>(&mut self, value: T) -> &mut Self {
        self.inner.value = value.as_ref().to_string();
        self
    }
}

impl AsRef<JsonValueString> for JsonValueString {
    fn as_ref(&self) -> &JsonValueString {
        self
    }
}

impl AsRef<JsonValueString> for JsonValueStringBuilder {
    fn as_ref(&self) -> &JsonValueString {
        &self.inner
    }
}
