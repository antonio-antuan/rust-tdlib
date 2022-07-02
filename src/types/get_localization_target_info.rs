use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns information about the current localization target. This is an offline request if only_local is true. Can be called before authorization
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetLocalizationTargetInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// If true, returns only locally available information without sending network requests

    #[serde(default)]
    only_local: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetLocalizationTargetInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetLocalizationTargetInfo {}

impl GetLocalizationTargetInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetLocalizationTargetInfoBuilder {
        let mut inner = GetLocalizationTargetInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getLocalizationTargetInfo".to_string();

        GetLocalizationTargetInfoBuilder { inner }
    }

    pub fn only_local(&self) -> bool {
        self.only_local
    }
}

#[doc(hidden)]
pub struct GetLocalizationTargetInfoBuilder {
    inner: GetLocalizationTargetInfo,
}

#[deprecated]
pub type RTDGetLocalizationTargetInfoBuilder = GetLocalizationTargetInfoBuilder;

impl GetLocalizationTargetInfoBuilder {
    pub fn build(&self) -> GetLocalizationTargetInfo {
        self.inner.clone()
    }

    pub fn only_local(&mut self, only_local: bool) -> &mut Self {
        self.inner.only_local = only_local;
        self
    }
}

impl AsRef<GetLocalizationTargetInfo> for GetLocalizationTargetInfo {
    fn as_ref(&self) -> &GetLocalizationTargetInfo {
        self
    }
}

impl AsRef<GetLocalizationTargetInfo> for GetLocalizationTargetInfoBuilder {
    fn as_ref(&self) -> &GetLocalizationTargetInfo {
        &self.inner
    }
}
