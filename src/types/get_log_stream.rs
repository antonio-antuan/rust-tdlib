use crate::errors::Result;
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetLogStreamBuilder {
        let mut inner = GetLogStream::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getLogStream".to_string();

        GetLogStreamBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetLogStreamBuilder {
    inner: GetLogStream,
}

#[deprecated]
pub type RTDGetLogStreamBuilder = GetLogStreamBuilder;

impl GetLogStreamBuilder {
    pub fn build(&self) -> GetLogStream {
        self.inner.clone()
    }
}

impl AsRef<GetLogStream> for GetLogStream {
    fn as_ref(&self) -> &GetLogStream {
        self
    }
}

impl AsRef<GetLogStream> for GetLogStreamBuilder {
    fn as_ref(&self) -> &GetLogStream {
        &self.inner
    }
}
