use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns menu button set by the bot for the given user; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMenuButton {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the user or 0 to get the default menu button
    user_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetMenuButton {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetMenuButton {}

impl GetMenuButton {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetMenuButtonBuilder {
        let mut inner = GetMenuButton::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getMenuButton".to_string();

        RTDGetMenuButtonBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }
}

#[doc(hidden)]
pub struct RTDGetMenuButtonBuilder {
    inner: GetMenuButton,
}

impl RTDGetMenuButtonBuilder {
    pub fn build(&self) -> GetMenuButton {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }
}

impl AsRef<GetMenuButton> for GetMenuButton {
    fn as_ref(&self) -> &GetMenuButton {
        self
    }
}

impl AsRef<GetMenuButton> for RTDGetMenuButtonBuilder {
    fn as_ref(&self) -> &GetMenuButton {
        &self.inner
    }
}
