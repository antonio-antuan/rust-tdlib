use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Toggles whether the supergroup is a forum; requires owner privileges in the supergroup. Discussion supergroups can't be converted to forums
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleSupergroupIsForum {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the supergroup

    #[serde(default)]
    supergroup_id: i64,
    /// New value of is_forum

    #[serde(default)]
    is_forum: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleSupergroupIsForum {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleSupergroupIsForum {}

impl ToggleSupergroupIsForum {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ToggleSupergroupIsForumBuilder {
        let mut inner = ToggleSupergroupIsForum::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleSupergroupIsForum".to_string();

        ToggleSupergroupIsForumBuilder { inner }
    }

    pub fn supergroup_id(&self) -> i64 {
        self.supergroup_id
    }

    pub fn is_forum(&self) -> bool {
        self.is_forum
    }
}

#[doc(hidden)]
pub struct ToggleSupergroupIsForumBuilder {
    inner: ToggleSupergroupIsForum,
}

#[deprecated]
pub type RTDToggleSupergroupIsForumBuilder = ToggleSupergroupIsForumBuilder;

impl ToggleSupergroupIsForumBuilder {
    pub fn build(&self) -> ToggleSupergroupIsForum {
        self.inner.clone()
    }

    pub fn supergroup_id(&mut self, supergroup_id: i64) -> &mut Self {
        self.inner.supergroup_id = supergroup_id;
        self
    }

    pub fn is_forum(&mut self, is_forum: bool) -> &mut Self {
        self.inner.is_forum = is_forum;
        self
    }
}

impl AsRef<ToggleSupergroupIsForum> for ToggleSupergroupIsForum {
    fn as_ref(&self) -> &ToggleSupergroupIsForum {
        self
    }
}

impl AsRef<ToggleSupergroupIsForum> for ToggleSupergroupIsForumBuilder {
    fn as_ref(&self) -> &ToggleSupergroupIsForum {
        &self.inner
    }
}
