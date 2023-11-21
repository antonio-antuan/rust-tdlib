use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sends log file for a call to Telegram servers
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendCallLog {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Call identifier

    #[serde(default)]
    call_id: i32,
    /// Call log file. Only inputFileLocal and inputFileGenerated are supported

    #[serde(skip_serializing_if = "InputFile::_is_default")]
    log_file: InputFile,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SendCallLog {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SendCallLog {}

impl SendCallLog {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SendCallLogBuilder {
        let mut inner = SendCallLog::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "sendCallLog".to_string();

        SendCallLogBuilder { inner }
    }

    pub fn call_id(&self) -> i32 {
        self.call_id
    }

    pub fn log_file(&self) -> &InputFile {
        &self.log_file
    }
}

#[doc(hidden)]
pub struct SendCallLogBuilder {
    inner: SendCallLog,
}

#[deprecated]
pub type RTDSendCallLogBuilder = SendCallLogBuilder;

impl SendCallLogBuilder {
    pub fn build(&self) -> SendCallLog {
        self.inner.clone()
    }

    pub fn call_id(&mut self, call_id: i32) -> &mut Self {
        self.inner.call_id = call_id;
        self
    }

    pub fn log_file<T: AsRef<InputFile>>(&mut self, log_file: T) -> &mut Self {
        self.inner.log_file = log_file.as_ref().clone();
        self
    }
}

impl AsRef<SendCallLog> for SendCallLog {
    fn as_ref(&self) -> &SendCallLog {
        self
    }
}

impl AsRef<SendCallLog> for SendCallLogBuilder {
    fn as_ref(&self) -> &SendCallLog {
        &self.inner
    }
}
