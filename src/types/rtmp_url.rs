use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents an RTMP url
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RtmpUrl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The URL
    url: String,
    /// Stream key
    stream_key: String,
}

impl RObject for RtmpUrl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RtmpUrl {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRtmpUrlBuilder {
        let mut inner = RtmpUrl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDRtmpUrlBuilder { inner }
    }

    pub fn url(&self) -> &String {
        &self.url
    }

    pub fn stream_key(&self) -> &String {
        &self.stream_key
    }
}

#[doc(hidden)]
pub struct RTDRtmpUrlBuilder {
    inner: RtmpUrl,
}

impl RTDRtmpUrlBuilder {
    pub fn build(&self) -> RtmpUrl {
        self.inner.clone()
    }

    pub fn url<T: AsRef<str>>(&mut self, url: T) -> &mut Self {
        self.inner.url = url.as_ref().to_string();
        self
    }

    pub fn stream_key<T: AsRef<str>>(&mut self, stream_key: T) -> &mut Self {
        self.inner.stream_key = stream_key.as_ref().to_string();
        self
    }
}

impl AsRef<RtmpUrl> for RtmpUrl {
    fn as_ref(&self) -> &RtmpUrl {
        self
    }
}

impl AsRef<RtmpUrl> for RTDRtmpUrlBuilder {
    fn as_ref(&self) -> &RtmpUrl {
        &self.inner
    }
}
