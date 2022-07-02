use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns t.me URLs recently visited by a newly registered user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetRecentlyVisitedTMeUrls {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Google Play referrer to identify the user

    #[serde(default)]
    referrer: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetRecentlyVisitedTMeUrls {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetRecentlyVisitedTMeUrls {}

impl GetRecentlyVisitedTMeUrls {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetRecentlyVisitedTMeUrlsBuilder {
        let mut inner = GetRecentlyVisitedTMeUrls::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getRecentlyVisitedTMeUrls".to_string();

        GetRecentlyVisitedTMeUrlsBuilder { inner }
    }

    pub fn referrer(&self) -> &String {
        &self.referrer
    }
}

#[doc(hidden)]
pub struct GetRecentlyVisitedTMeUrlsBuilder {
    inner: GetRecentlyVisitedTMeUrls,
}

#[deprecated]
pub type RTDGetRecentlyVisitedTMeUrlsBuilder = GetRecentlyVisitedTMeUrlsBuilder;

impl GetRecentlyVisitedTMeUrlsBuilder {
    pub fn build(&self) -> GetRecentlyVisitedTMeUrls {
        self.inner.clone()
    }

    pub fn referrer<T: AsRef<str>>(&mut self, referrer: T) -> &mut Self {
        self.inner.referrer = referrer.as_ref().to_string();
        self
    }
}

impl AsRef<GetRecentlyVisitedTMeUrls> for GetRecentlyVisitedTMeUrls {
    fn as_ref(&self) -> &GetRecentlyVisitedTMeUrls {
        self
    }
}

impl AsRef<GetRecentlyVisitedTMeUrls> for GetRecentlyVisitedTMeUrlsBuilder {
    fn as_ref(&self) -> &GetRecentlyVisitedTMeUrls {
        &self.inner
    }
}
