use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Adds a message to TDLib internal log. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddLogMessage {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The minimum verbosity level needed for the message to be logged, 0-1023
    verbosity_level: i32,
    /// Text of a message to log
    text: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AddLogMessage {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AddLogMessage {}

impl AddLogMessage {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDAddLogMessageBuilder {
        let mut inner = AddLogMessage::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "addLogMessage".to_string();

        RTDAddLogMessageBuilder { inner }
    }

    pub fn verbosity_level(&self) -> i32 {
        self.verbosity_level
    }

    pub fn text(&self) -> &String {
        &self.text
    }
}

#[doc(hidden)]
pub struct RTDAddLogMessageBuilder {
    inner: AddLogMessage,
}

impl RTDAddLogMessageBuilder {
    pub fn build(&self) -> AddLogMessage {
        self.inner.clone()
    }

    pub fn verbosity_level(&mut self, verbosity_level: i32) -> &mut Self {
        self.inner.verbosity_level = verbosity_level;
        self
    }

    pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().to_string();
        self
    }
}

impl AsRef<AddLogMessage> for AddLogMessage {
    fn as_ref(&self) -> &AddLogMessage {
        self
    }
}

impl AsRef<AddLogMessage> for RTDAddLogMessageBuilder {
    fn as_ref(&self) -> &AddLogMessage {
        &self.inner
    }
}
