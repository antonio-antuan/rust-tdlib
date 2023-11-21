use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns list of custom emojis, which can be used as forum topic icon by all users
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetForumTopicDefaultIcons {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetForumTopicDefaultIcons {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetForumTopicDefaultIcons {}

impl GetForumTopicDefaultIcons {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetForumTopicDefaultIconsBuilder {
        let mut inner = GetForumTopicDefaultIcons::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getForumTopicDefaultIcons".to_string();

        GetForumTopicDefaultIconsBuilder { inner }
    }
}

#[doc(hidden)]
pub struct GetForumTopicDefaultIconsBuilder {
    inner: GetForumTopicDefaultIcons,
}

#[deprecated]
pub type RTDGetForumTopicDefaultIconsBuilder = GetForumTopicDefaultIconsBuilder;

impl GetForumTopicDefaultIconsBuilder {
    pub fn build(&self) -> GetForumTopicDefaultIcons {
        self.inner.clone()
    }
}

impl AsRef<GetForumTopicDefaultIcons> for GetForumTopicDefaultIcons {
    fn as_ref(&self) -> &GetForumTopicDefaultIcons {
        self
    }
}

impl AsRef<GetForumTopicDefaultIcons> for GetForumTopicDefaultIconsBuilder {
    fn as_ref(&self) -> &GetForumTopicDefaultIcons {
        &self.inner
    }
}
