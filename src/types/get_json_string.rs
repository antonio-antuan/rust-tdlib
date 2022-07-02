use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Converts a JsonValue object to corresponding JSON-serialized string. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetJsonString {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The JsonValue object

    #[serde(skip_serializing_if = "JsonValue::_is_default")]
    json_value: JsonValue,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetJsonString {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetJsonString {}

impl GetJsonString {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetJsonStringBuilder {
        let mut inner = GetJsonString::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getJsonString".to_string();

        GetJsonStringBuilder { inner }
    }

    pub fn json_value(&self) -> &JsonValue {
        &self.json_value
    }
}

#[doc(hidden)]
pub struct GetJsonStringBuilder {
    inner: GetJsonString,
}

#[deprecated]
pub type RTDGetJsonStringBuilder = GetJsonStringBuilder;

impl GetJsonStringBuilder {
    pub fn build(&self) -> GetJsonString {
        self.inner.clone()
    }

    pub fn json_value<T: AsRef<JsonValue>>(&mut self, json_value: T) -> &mut Self {
        self.inner.json_value = json_value.as_ref().clone();
        self
    }
}

impl AsRef<GetJsonString> for GetJsonString {
    fn as_ref(&self) -> &GetJsonString {
        self
    }
}

impl AsRef<GetJsonString> for GetJsonStringBuilder {
    fn as_ref(&self) -> &GetJsonString {
        &self.inner
    }
}
