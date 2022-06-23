use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns RTMP URL for streaming to the chat; requires creator privileges
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetVideoChatRtmpUrl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat identifier
    chat_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetVideoChatRtmpUrl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetVideoChatRtmpUrl {}

impl GetVideoChatRtmpUrl {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetVideoChatRtmpUrlBuilder {
        let mut inner = GetVideoChatRtmpUrl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getVideoChatRtmpUrl".to_string();

        RTDGetVideoChatRtmpUrlBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct RTDGetVideoChatRtmpUrlBuilder {
    inner: GetVideoChatRtmpUrl,
}

impl RTDGetVideoChatRtmpUrlBuilder {
    pub fn build(&self) -> GetVideoChatRtmpUrl {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<GetVideoChatRtmpUrl> for GetVideoChatRtmpUrl {
    fn as_ref(&self) -> &GetVideoChatRtmpUrl {
        self
    }
}

impl AsRef<GetVideoChatRtmpUrl> for RTDGetVideoChatRtmpUrlBuilder {
    fn as_ref(&self) -> &GetVideoChatRtmpUrl {
        &self.inner
    }
}
