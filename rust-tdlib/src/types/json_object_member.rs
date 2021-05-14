use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents one member of a JSON object
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JsonObjectMember {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Member's key
    key: String,
    /// Member's value

    #[serde(skip_serializing_if = "JsonValue::_is_default")]
    value: JsonValue,
}

impl RObject for JsonObjectMember {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl JsonObjectMember {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDJsonObjectMemberBuilder {
        let mut inner = JsonObjectMember::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDJsonObjectMemberBuilder { inner }
    }

    pub fn key(&self) -> &String {
        &self.key
    }

    pub fn value(&self) -> &JsonValue {
        &self.value
    }
}

#[doc(hidden)]
pub struct RTDJsonObjectMemberBuilder {
    inner: JsonObjectMember,
}

impl RTDJsonObjectMemberBuilder {
    pub fn build(&self) -> JsonObjectMember {
        self.inner.clone()
    }

    pub fn key<T: AsRef<str>>(&mut self, key: T) -> &mut Self {
        self.inner.key = key.as_ref().to_string();
        self
    }

    pub fn value<T: AsRef<JsonValue>>(&mut self, value: T) -> &mut Self {
        self.inner.value = value.as_ref().clone();
        self
    }
}

impl AsRef<JsonObjectMember> for JsonObjectMember {
    fn as_ref(&self) -> &JsonObjectMember {
        self
    }
}

impl AsRef<JsonObjectMember> for RTDJsonObjectMemberBuilder {
    fn as_ref(&self) -> &JsonObjectMember {
        &self.inner
    }
}
