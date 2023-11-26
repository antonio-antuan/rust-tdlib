use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about available group call streams
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetGroupCallStreams {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group call identifier

    #[serde(default)]
    group_call_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetGroupCallStreams {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetGroupCallStreams {}

impl GetGroupCallStreams {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetGroupCallStreamsBuilder {
        let mut inner = GetGroupCallStreams::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getGroupCallStreams".to_string();

        GetGroupCallStreamsBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }
}

#[doc(hidden)]
pub struct GetGroupCallStreamsBuilder {
    inner: GetGroupCallStreams,
}

#[deprecated]
pub type RTDGetGroupCallStreamsBuilder = GetGroupCallStreamsBuilder;

impl GetGroupCallStreamsBuilder {
    pub fn build(&self) -> GetGroupCallStreams {
        self.inner.clone()
    }

    pub fn group_call_id(&mut self, group_call_id: i32) -> &mut Self {
        self.inner.group_call_id = group_call_id;
        self
    }
}

impl AsRef<GetGroupCallStreams> for GetGroupCallStreams {
    fn as_ref(&self) -> &GetGroupCallStreams {
        self
    }
}

impl AsRef<GetGroupCallStreams> for GetGroupCallStreamsBuilder {
    fn as_ref(&self) -> &GetGroupCallStreams {
        &self.inner
    }
}
