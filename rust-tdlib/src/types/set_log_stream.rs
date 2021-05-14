use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Sets new log stream for internal logging of TDLib. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetLogStream {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// New log stream

    #[serde(skip_serializing_if = "LogStream::_is_default")]
    log_stream: LogStream,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetLogStream {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetLogStream {}

impl SetLogStream {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetLogStreamBuilder {
        let mut inner = SetLogStream::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setLogStream".to_string();

        RTDSetLogStreamBuilder { inner }
    }

    pub fn log_stream(&self) -> &LogStream {
        &self.log_stream
    }
}

#[doc(hidden)]
pub struct RTDSetLogStreamBuilder {
    inner: SetLogStream,
}

impl RTDSetLogStreamBuilder {
    pub fn build(&self) -> SetLogStream {
        self.inner.clone()
    }

    pub fn log_stream<T: AsRef<LogStream>>(&mut self, log_stream: T) -> &mut Self {
        self.inner.log_stream = log_stream.as_ref().clone();
        self
    }
}

impl AsRef<SetLogStream> for SetLogStream {
    fn as_ref(&self) -> &SetLogStream {
        self
    }
}

impl AsRef<SetLogStream> for RTDSetLogStreamBuilder {
    fn as_ref(&self) -> &SetLogStream {
        &self.inner
    }
}
