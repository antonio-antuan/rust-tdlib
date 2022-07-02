use crate::errors::Result;
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
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ResetBackgroundsBuilder {
        let mut inner = ResetBackgrounds::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "resetBackgrounds".to_string();

        ResetBackgroundsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct ResetBackgroundsBuilder {
    inner: ResetBackgrounds,
}

#[deprecated]
pub type RTDResetBackgroundsBuilder = ResetBackgroundsBuilder;

impl ResetBackgroundsBuilder {
    pub fn build(&self) -> ResetBackgrounds {
        self.inner.clone()
    }
}

impl AsRef<ResetBackgrounds> for ResetBackgrounds {
    fn as_ref(&self) -> &ResetBackgrounds {
        self
    }
}

impl AsRef<ResetBackgrounds> for ResetBackgroundsBuilder {
    fn as_ref(&self) -> &ResetBackgrounds {
        &self.inner
    }
}
