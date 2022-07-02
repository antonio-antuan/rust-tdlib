use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Saves application log event on the server. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SaveApplicationLogEvent {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Event type

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(default)]
    type_: String,
    /// Optional chat identifier, associated with the event

    #[serde(default)]
    chat_id: i64,
    /// The log event data

    #[serde(skip_serializing_if = "JsonValue::_is_default")]
    data: JsonValue,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SaveApplicationLogEvent {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SaveApplicationLogEvent {}

impl SaveApplicationLogEvent {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SaveApplicationLogEventBuilder {
        let mut inner = SaveApplicationLogEvent::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "saveApplicationLogEvent".to_string();

        SaveApplicationLogEventBuilder { inner }
    }

    pub fn type_(&self) -> &String {
        &self.type_
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }

    pub fn data(&self) -> &JsonValue {
        &self.data
    }
}

#[doc(hidden)]
pub struct SaveApplicationLogEventBuilder {
    inner: SaveApplicationLogEvent,
}

#[deprecated]
pub type RTDSaveApplicationLogEventBuilder = SaveApplicationLogEventBuilder;

impl SaveApplicationLogEventBuilder {
    pub fn build(&self) -> SaveApplicationLogEvent {
        self.inner.clone()
    }

    pub fn type_<T: AsRef<str>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().to_string();
        self
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }

    pub fn data<T: AsRef<JsonValue>>(&mut self, data: T) -> &mut Self {
        self.inner.data = data.as_ref().clone();
        self
    }
}

impl AsRef<SaveApplicationLogEvent> for SaveApplicationLogEvent {
    fn as_ref(&self) -> &SaveApplicationLogEvent {
        self
    }
}

impl AsRef<SaveApplicationLogEvent> for SaveApplicationLogEventBuilder {
    fn as_ref(&self) -> &SaveApplicationLogEvent {
        &self.inner
    }
}
