use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns a file with a segment of a group call stream in a modified OGG format for audio or MPEG-4 format for video
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetGroupCallStreamSegment {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group call identifier

    #[serde(default)]
    group_call_id: i32,
    /// Point in time when the stream segment begins; Unix timestamp in milliseconds

    #[serde(default)]
    time_offset: i64,
    /// Segment duration scale; 0-1. Segment's duration is 1000/(2**scale) milliseconds

    #[serde(default)]
    scale: i32,
    /// Identifier of an audio/video channel to get as received from tgcalls

    #[serde(default)]
    channel_id: i32,
    /// Video quality as received from tgcalls; pass null to get the worst available quality

    #[serde(skip_serializing_if = "GroupCallVideoQuality::_is_default")]
    video_quality: GroupCallVideoQuality,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetGroupCallStreamSegment {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetGroupCallStreamSegment {}

impl GetGroupCallStreamSegment {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetGroupCallStreamSegmentBuilder {
        let mut inner = GetGroupCallStreamSegment::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getGroupCallStreamSegment".to_string();

        GetGroupCallStreamSegmentBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }

    pub fn time_offset(&self) -> i64 {
        self.time_offset
    }

    pub fn scale(&self) -> i32 {
        self.scale
    }

    pub fn channel_id(&self) -> i32 {
        self.channel_id
    }

    pub fn video_quality(&self) -> &GroupCallVideoQuality {
        &self.video_quality
    }
}

#[doc(hidden)]
pub struct GetGroupCallStreamSegmentBuilder {
    inner: GetGroupCallStreamSegment,
}

#[deprecated]
pub type RTDGetGroupCallStreamSegmentBuilder = GetGroupCallStreamSegmentBuilder;

impl GetGroupCallStreamSegmentBuilder {
    pub fn build(&self) -> GetGroupCallStreamSegment {
        self.inner.clone()
    }

    pub fn group_call_id(&mut self, group_call_id: i32) -> &mut Self {
        self.inner.group_call_id = group_call_id;
        self
    }

    pub fn time_offset(&mut self, time_offset: i64) -> &mut Self {
        self.inner.time_offset = time_offset;
        self
    }

    pub fn scale(&mut self, scale: i32) -> &mut Self {
        self.inner.scale = scale;
        self
    }

    pub fn channel_id(&mut self, channel_id: i32) -> &mut Self {
        self.inner.channel_id = channel_id;
        self
    }

    pub fn video_quality<T: AsRef<GroupCallVideoQuality>>(
        &mut self,
        video_quality: T,
    ) -> &mut Self {
        self.inner.video_quality = video_quality.as_ref().clone();
        self
    }
}

impl AsRef<GetGroupCallStreamSegment> for GetGroupCallStreamSegment {
    fn as_ref(&self) -> &GetGroupCallStreamSegment {
        self
    }
}

impl AsRef<GetGroupCallStreamSegment> for GetGroupCallStreamSegmentBuilder {
    fn as_ref(&self) -> &GetGroupCallStreamSegment {
        &self.inner
    }
}
