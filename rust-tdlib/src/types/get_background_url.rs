use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Constructs a persistent HTTP URL for a background
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBackgroundUrl {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Background name
    name: String,
    /// Background type

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "BackgroundType::_is_default")]
    type_: BackgroundType,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetBackgroundUrl {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetBackgroundUrl {}

impl GetBackgroundUrl {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetBackgroundUrlBuilder {
        let mut inner = GetBackgroundUrl::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getBackgroundUrl".to_string();

        RTDGetBackgroundUrlBuilder { inner }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn type_(&self) -> &BackgroundType {
        &self.type_
    }
}

#[doc(hidden)]
pub struct RTDGetBackgroundUrlBuilder {
    inner: GetBackgroundUrl,
}

impl RTDGetBackgroundUrlBuilder {
    pub fn build(&self) -> GetBackgroundUrl {
        self.inner.clone()
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }

    pub fn type_<T: AsRef<BackgroundType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }
}

impl AsRef<GetBackgroundUrl> for GetBackgroundUrl {
    fn as_ref(&self) -> &GetBackgroundUrl {
        self
    }
}

impl AsRef<GetBackgroundUrl> for RTDGetBackgroundUrlBuilder {
    fn as_ref(&self) -> &GetBackgroundUrl {
        &self.inner
    }
}
