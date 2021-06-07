use crate::errors::*;
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
    #[serde(rename(
        serialize = "getApplicationConfig",
        deserialize = "getApplicationConfig"
    ))]
    GetApplicationConfig(GetApplicationConfig),
    /// Converts a JSON-serialized string to corresponding JsonValue object. Can be called synchronously
    #[serde(rename(serialize = "getJsonValue", deserialize = "getJsonValue"))]
    GetJsonValue(GetJsonValue),
    /// Represents a JSON array
    #[serde(rename(serialize = "jsonValueArray", deserialize = "jsonValueArray"))]
    Array(JsonValueArray),
    /// Represents a boolean JSON value
    #[serde(rename(serialize = "jsonValueBoolean", deserialize = "jsonValueBoolean"))]
    Boolean(JsonValueBoolean),
    /// Represents a null JSON value
    #[serde(rename(serialize = "jsonValueNull", deserialize = "jsonValueNull"))]
    Null(JsonValueNull),
    /// Represents a numeric JSON value
    #[serde(rename(serialize = "jsonValueNumber", deserialize = "jsonValueNumber"))]
    Number(JsonValueNumber),
    /// Represents a JSON object
    #[serde(rename(serialize = "jsonValueObject", deserialize = "jsonValueObject"))]
    Object(JsonValueObject),
    /// Represents a string JSON value
    #[serde(rename(serialize = "jsonValueString", deserialize = "jsonValueString"))]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDJsonValueArrayBuilder {
        let mut inner = JsonValueArray::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDJsonValueArrayBuilder { inner }
    }

    pub fn values(&self) -> &Vec<JsonValue> {
        &self.values
    }
}

#[doc(hidden)]
pub struct RTDJsonValueArrayBuilder {
    inner: JsonValueArray,
}

impl RTDJsonValueArrayBuilder {
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

impl AsRef<JsonValueArray> for RTDJsonValueArrayBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDJsonValueBooleanBuilder {
        let mut inner = JsonValueBoolean::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDJsonValueBooleanBuilder { inner }
    }

    pub fn value(&self) -> bool {
        self.value
    }
}

#[doc(hidden)]
pub struct RTDJsonValueBooleanBuilder {
    inner: JsonValueBoolean,
}

impl RTDJsonValueBooleanBuilder {
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

impl AsRef<JsonValueBoolean> for RTDJsonValueBooleanBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDJsonValueNullBuilder {
        let mut inner = JsonValueNull::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDJsonValueNullBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDJsonValueNullBuilder {
    inner: JsonValueNull,
}

impl RTDJsonValueNullBuilder {
    pub fn build(&self) -> JsonValueNull {
        self.inner.clone()
    }
}

impl AsRef<JsonValueNull> for JsonValueNull {
    fn as_ref(&self) -> &JsonValueNull {
        self
    }
}

impl AsRef<JsonValueNull> for RTDJsonValueNullBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDJsonValueNumberBuilder {
        let mut inner = JsonValueNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDJsonValueNumberBuilder { inner }
    }

    pub fn value(&self) -> f32 {
        self.value
    }
}

#[doc(hidden)]
pub struct RTDJsonValueNumberBuilder {
    inner: JsonValueNumber,
}

impl RTDJsonValueNumberBuilder {
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

impl AsRef<JsonValueNumber> for RTDJsonValueNumberBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDJsonValueObjectBuilder {
        let mut inner = JsonValueObject::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDJsonValueObjectBuilder { inner }
    }

    pub fn members(&self) -> &Vec<JsonObjectMember> {
        &self.members
    }
}

#[doc(hidden)]
pub struct RTDJsonValueObjectBuilder {
    inner: JsonValueObject,
}

impl RTDJsonValueObjectBuilder {
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

impl AsRef<JsonValueObject> for RTDJsonValueObjectBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDJsonValueStringBuilder {
        let mut inner = JsonValueString::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDJsonValueStringBuilder { inner }
    }

    pub fn value(&self) -> &String {
        &self.value
    }
}

#[doc(hidden)]
pub struct RTDJsonValueStringBuilder {
    inner: JsonValueString,
}

impl RTDJsonValueStringBuilder {
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

impl AsRef<JsonValueString> for RTDJsonValueStringBuilder {
    fn as_ref(&self) -> &JsonValueString {
        &self.inner
    }
}
