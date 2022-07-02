use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns backgrounds installed by the user
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetBackgrounds {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// True, if the backgrounds must be ordered for dark theme

    #[serde(default)]
    for_dark_theme: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetBackgrounds {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetBackgrounds {}

impl GetBackgrounds {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetBackgroundsBuilder {
        let mut inner = GetBackgrounds::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getBackgrounds".to_string();

        GetBackgroundsBuilder { inner }
    }

    pub fn for_dark_theme(&self) -> bool {
        self.for_dark_theme
    }
}

#[doc(hidden)]
pub struct GetBackgroundsBuilder {
    inner: GetBackgrounds,
}

#[deprecated]
pub type RTDGetBackgroundsBuilder = GetBackgroundsBuilder;

impl GetBackgroundsBuilder {
    pub fn build(&self) -> GetBackgrounds {
        self.inner.clone()
    }

    pub fn for_dark_theme(&mut self, for_dark_theme: bool) -> &mut Self {
        self.inner.for_dark_theme = for_dark_theme;
        self
    }
}

impl AsRef<GetBackgrounds> for GetBackgrounds {
    fn as_ref(&self) -> &GetBackgrounds {
        self
    }
}

impl AsRef<GetBackgrounds> for GetBackgroundsBuilder {
    fn as_ref(&self) -> &GetBackgrounds {
        &self.inner
    }
}
