use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes an available stream in a group call
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallStream {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of an audio/video channel
    channel_id: i32,
    /// Scale of segment durations in the stream. The duration is 1000/(2**scale) milliseconds
    scale: i32,
    /// Point in time when the stream currently ends; Unix timestamp in milliseconds
    time_offset: i64,
}

impl RObject for GroupCallStream {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl GroupCallStream {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGroupCallStreamBuilder {
        let mut inner = GroupCallStream::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDGroupCallStreamBuilder { inner }
    }

    pub fn channel_id(&self) -> i32 {
        self.channel_id
    }

    pub fn scale(&self) -> i32 {
        self.scale
    }

    pub fn time_offset(&self) -> i64 {
        self.time_offset
    }
}

#[doc(hidden)]
pub struct RTDGroupCallStreamBuilder {
    inner: GroupCallStream,
}

impl RTDGroupCallStreamBuilder {
    pub fn build(&self) -> GroupCallStream {
        self.inner.clone()
    }

    pub fn channel_id(&mut self, channel_id: i32) -> &mut Self {
        self.inner.channel_id = channel_id;
        self
    }

    pub fn scale(&mut self, scale: i32) -> &mut Self {
        self.inner.scale = scale;
        self
    }

    pub fn time_offset(&mut self, time_offset: i64) -> &mut Self {
        self.inner.time_offset = time_offset;
        self
    }
}

impl AsRef<GroupCallStream> for GroupCallStream {
    fn as_ref(&self) -> &GroupCallStream {
        self
    }
}

impl AsRef<GroupCallStream> for RTDGroupCallStreamBuilder {
    fn as_ref(&self) -> &GroupCallStream {
        &self.inner
    }
}
