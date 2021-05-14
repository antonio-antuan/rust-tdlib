use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns saved animations
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetSavedAnimations {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetSavedAnimations {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetSavedAnimations {}

impl GetSavedAnimations {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetSavedAnimationsBuilder {
        let mut inner = GetSavedAnimations::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getSavedAnimations".to_string();

        RTDGetSavedAnimationsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDGetSavedAnimationsBuilder {
    inner: GetSavedAnimations,
}

impl RTDGetSavedAnimationsBuilder {
    pub fn build(&self) -> GetSavedAnimations {
        self.inner.clone()
    }
}

impl AsRef<GetSavedAnimations> for GetSavedAnimations {
    fn as_ref(&self) -> &GetSavedAnimations {
        self
    }
}

impl AsRef<GetSavedAnimations> for RTDGetSavedAnimationsBuilder {
    fn as_ref(&self) -> &GetSavedAnimations {
        &self.inner
    }
}
