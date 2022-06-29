use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Sets menu button for the given user or for all users; for bots only
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetMenuButton {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the user or 0 to set menu button for all users

    #[serde(default)]
    user_id: i64,
    /// New menu button
    menu_button: BotMenuButton,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetMenuButton {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetMenuButton {}

impl SetMenuButton {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDSetMenuButtonBuilder {
        let mut inner = SetMenuButton::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setMenuButton".to_string();

        RTDSetMenuButtonBuilder { inner }
    }

    pub fn user_id(&self) -> i64 {
        self.user_id
    }

    pub fn menu_button(&self) -> &BotMenuButton {
        &self.menu_button
    }
}

#[doc(hidden)]
pub struct RTDSetMenuButtonBuilder {
    inner: SetMenuButton,
}

impl RTDSetMenuButtonBuilder {
    pub fn build(&self) -> SetMenuButton {
        self.inner.clone()
    }

    pub fn user_id(&mut self, user_id: i64) -> &mut Self {
        self.inner.user_id = user_id;
        self
    }

    pub fn menu_button<T: AsRef<BotMenuButton>>(&mut self, menu_button: T) -> &mut Self {
        self.inner.menu_button = menu_button.as_ref().clone();
        self
    }
}

impl AsRef<SetMenuButton> for SetMenuButton {
    fn as_ref(&self) -> &SetMenuButton {
        self
    }
}

impl AsRef<SetMenuButton> for RTDSetMenuButtonBuilder {
    fn as_ref(&self) -> &SetMenuButton {
        &self.inner
    }
}
