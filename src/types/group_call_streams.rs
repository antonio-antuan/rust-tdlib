use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of group call streams
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallStreams {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// A list of group call streams
    streams: Vec<GroupCallStream>,
}

impl RObject for GroupCallStreams {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl GroupCallStreams {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGroupCallStreamsBuilder {
        let mut inner = GroupCallStreams::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDGroupCallStreamsBuilder { inner }
    }

    pub fn streams(&self) -> &Vec<GroupCallStream> {
        &self.streams
    }
}

#[doc(hidden)]
pub struct RTDGroupCallStreamsBuilder {
    inner: GroupCallStreams,
}

impl RTDGroupCallStreamsBuilder {
    pub fn build(&self) -> GroupCallStreams {
        self.inner.clone()
    }

    pub fn streams(&mut self, streams: Vec<GroupCallStream>) -> &mut Self {
        self.inner.streams = streams;
        self
    }
}

impl AsRef<GroupCallStreams> for GroupCallStreams {
    fn as_ref(&self) -> &GroupCallStreams {
        self
    }
}

impl AsRef<GroupCallStreams> for RTDGroupCallStreamsBuilder {
    fn as_ref(&self) -> &GroupCallStreams {
        &self.inner
    }
}
