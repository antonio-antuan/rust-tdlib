use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Replaces the current RTMP URL for streaming to the chat; requires creator privileges
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReplaceVideoChatRtmpUrl {
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

impl RObject for ReplaceVideoChatRtmpUrl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ReplaceVideoChatRtmpUrl {}

impl ReplaceVideoChatRtmpUrl {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDReplaceVideoChatRtmpUrlBuilder {
        let mut inner = ReplaceVideoChatRtmpUrl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "replaceVideoChatRtmpUrl".to_string();

        RTDReplaceVideoChatRtmpUrlBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct RTDReplaceVideoChatRtmpUrlBuilder {
    inner: ReplaceVideoChatRtmpUrl,
}

impl RTDReplaceVideoChatRtmpUrlBuilder {
    pub fn build(&self) -> ReplaceVideoChatRtmpUrl {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<ReplaceVideoChatRtmpUrl> for ReplaceVideoChatRtmpUrl {
    fn as_ref(&self) -> &ReplaceVideoChatRtmpUrl {
        self
    }
}

impl AsRef<ReplaceVideoChatRtmpUrl> for RTDReplaceVideoChatRtmpUrlBuilder {
    fn as_ref(&self) -> &ReplaceVideoChatRtmpUrl {
        &self.inner
    }
}
