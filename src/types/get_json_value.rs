use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Converts a JSON-serialized string to corresponding JsonValue object. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetJsonValue {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The JSON-serialized string

    #[serde(default)]
    json: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetJsonValue {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDJsonValue for GetJsonValue {}

impl RFunction for GetJsonValue {}

impl GetJsonValue {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetJsonValueBuilder {
        let mut inner = GetJsonValue::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getJsonValue".to_string();

        GetJsonValueBuilder { inner }
    }

    pub fn json(&self) -> &String {
        &self.json
    }
}

#[doc(hidden)]
pub struct GetJsonValueBuilder {
    inner: GetJsonValue,
}

#[deprecated]
pub type RTDGetJsonValueBuilder = GetJsonValueBuilder;

impl GetJsonValueBuilder {
    pub fn build(&self) -> GetJsonValue {
        self.inner.clone()
    }

    pub fn json<T: AsRef<str>>(&mut self, json: T) -> &mut Self {
        self.inner.json = json.as_ref().to_string();
        self
    }
}

impl AsRef<GetJsonValue> for GetJsonValue {
    fn as_ref(&self) -> &GetJsonValue {
        self
    }
}

impl AsRef<GetJsonValue> for GetJsonValueBuilder {
    fn as_ref(&self) -> &GetJsonValue {
        &self.inner
    }
}
