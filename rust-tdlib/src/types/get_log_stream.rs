use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns information about currently used log stream for internal logging of TDLib. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLogStream {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetLogStream {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDLogStream for GetLogStream {}

impl RFunction for GetLogStream {}

impl GetLogStream {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetLogStreamBuilder {
        let mut inner = GetLogStream::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getLogStream".to_string();

        RTDGetLogStreamBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDGetLogStreamBuilder {
    inner: GetLogStream,
}

impl RTDGetLogStreamBuilder {
    pub fn build(&self) -> GetLogStream {
        self.inner.clone()
    }
}

impl AsRef<GetLogStream> for GetLogStream {
    fn as_ref(&self) -> &GetLogStream {
        self
    }
}

impl AsRef<GetLogStream> for RTDGetLogStreamBuilder {
    fn as_ref(&self) -> &GetLogStream {
        &self.inner
    }
}
