use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains information about a group call participant's video channel
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupCallParticipantVideoInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of synchronization source groups of the video

    #[serde(default)]
    source_groups: Vec<GroupCallVideoSourceGroup>,
    /// Video channel endpoint identifier

    #[serde(default)]
    endpoint_id: String,
    /// True if the video is paused. This flag needs to be ignored, if new video frames are received

    #[serde(default)]
    is_paused: bool,
}

impl RObject for GroupCallParticipantVideoInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl GroupCallParticipantVideoInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GroupCallParticipantVideoInfoBuilder {
        let mut inner = GroupCallParticipantVideoInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        GroupCallParticipantVideoInfoBuilder { inner }
    }

    pub fn source_groups(&self) -> &Vec<GroupCallVideoSourceGroup> {
        &self.source_groups
    }

    pub fn endpoint_id(&self) -> &String {
        &self.endpoint_id
    }

    pub fn is_paused(&self) -> bool {
        self.is_paused
    }
}

#[doc(hidden)]
pub struct GroupCallParticipantVideoInfoBuilder {
    inner: GroupCallParticipantVideoInfo,
}

#[deprecated]
pub type RTDGroupCallParticipantVideoInfoBuilder = GroupCallParticipantVideoInfoBuilder;

impl GroupCallParticipantVideoInfoBuilder {
    pub fn build(&self) -> GroupCallParticipantVideoInfo {
        self.inner.clone()
    }

    pub fn source_groups(&mut self, source_groups: Vec<GroupCallVideoSourceGroup>) -> &mut Self {
        self.inner.source_groups = source_groups;
        self
    }

    pub fn endpoint_id<T: AsRef<str>>(&mut self, endpoint_id: T) -> &mut Self {
        self.inner.endpoint_id = endpoint_id.as_ref().to_string();
        self
    }

    pub fn is_paused(&mut self, is_paused: bool) -> &mut Self {
        self.inner.is_paused = is_paused;
        self
    }
}

impl AsRef<GroupCallParticipantVideoInfo> for GroupCallParticipantVideoInfo {
    fn as_ref(&self) -> &GroupCallParticipantVideoInfo {
        self
    }
}

impl AsRef<GroupCallParticipantVideoInfo> for GroupCallParticipantVideoInfoBuilder {
    fn as_ref(&self) -> &GroupCallParticipantVideoInfo {
        &self.inner
    }
}
