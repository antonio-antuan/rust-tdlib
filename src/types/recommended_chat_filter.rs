use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Describes a recommended chat filter
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RecommendedChatFilter {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    /// The chat filter
    filter: ChatFilter,
    /// Describes a recommended chat filter
    description: String,
}

impl RObject for RecommendedChatFilter {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "recommendedChatFilter"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl RecommendedChatFilter {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDRecommendedChatFilterBuilder {
        let mut inner = RecommendedChatFilter::default();
        inner.td_name = "recommendedChatFilter".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        RTDRecommendedChatFilterBuilder { inner }
    }

    pub fn filter(&self) -> &ChatFilter {
        &self.filter
    }

    pub fn description(&self) -> &String {
        &self.description
    }
}

#[doc(hidden)]
pub struct RTDRecommendedChatFilterBuilder {
    inner: RecommendedChatFilter,
}

impl RTDRecommendedChatFilterBuilder {
    pub fn build(&self) -> RecommendedChatFilter {
        self.inner.clone()
    }

    pub fn filter<T: AsRef<ChatFilter>>(&mut self, filter: T) -> &mut Self {
        self.inner.filter = filter.as_ref().clone();
        self
    }

    pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
        self.inner.description = description.as_ref().to_string();
        self
    }
}

impl AsRef<RecommendedChatFilter> for RecommendedChatFilter {
    fn as_ref(&self) -> &RecommendedChatFilter {
        self
    }
}

impl AsRef<RecommendedChatFilter> for RTDRecommendedChatFilterBuilder {
    fn as_ref(&self) -> &RecommendedChatFilter {
        &self.inner
    }
}
