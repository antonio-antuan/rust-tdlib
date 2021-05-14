use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Resets list of installed backgrounds to its default value
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ResetBackgrounds {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ResetBackgrounds {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ResetBackgrounds {}

impl ResetBackgrounds {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDResetBackgroundsBuilder {
        let mut inner = ResetBackgrounds::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "resetBackgrounds".to_string();

        RTDResetBackgroundsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDResetBackgroundsBuilder {
    inner: ResetBackgrounds,
}

impl RTDResetBackgroundsBuilder {
    pub fn build(&self) -> ResetBackgrounds {
        self.inner.clone()
    }
}

impl AsRef<ResetBackgrounds> for ResetBackgrounds {
    fn as_ref(&self) -> &ResetBackgrounds {
        self
    }
}

impl AsRef<ResetBackgrounds> for RTDResetBackgroundsBuilder {
    fn as_ref(&self) -> &ResetBackgrounds {
        &self.inner
    }
}
