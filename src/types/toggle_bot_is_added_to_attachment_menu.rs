use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Adds or removes a bot to attachment menu. Bot can be added to attachment menu, only if userTypeBot.can_be_added_to_attachment_menu == true
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ToggleBotIsAddedToAttachmentMenu {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Bot's user identifier
    bot_user_id: i64,
    /// Pass true to add the bot to attachment menu; pass false to remove the bot from attachment menu
    is_added: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for ToggleBotIsAddedToAttachmentMenu {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for ToggleBotIsAddedToAttachmentMenu {}

impl ToggleBotIsAddedToAttachmentMenu {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDToggleBotIsAddedToAttachmentMenuBuilder {
        let mut inner = ToggleBotIsAddedToAttachmentMenu::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "toggleBotIsAddedToAttachmentMenu".to_string();

        RTDToggleBotIsAddedToAttachmentMenuBuilder { inner }
    }

    pub fn bot_user_id(&self) -> i64 {
        self.bot_user_id
    }

    pub fn is_added(&self) -> bool {
        self.is_added
    }
}

#[doc(hidden)]
pub struct RTDToggleBotIsAddedToAttachmentMenuBuilder {
    inner: ToggleBotIsAddedToAttachmentMenu,
}

impl RTDToggleBotIsAddedToAttachmentMenuBuilder {
    pub fn build(&self) -> ToggleBotIsAddedToAttachmentMenu {
        self.inner.clone()
    }

    pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
        self.inner.bot_user_id = bot_user_id;
        self
    }

    pub fn is_added(&mut self, is_added: bool) -> &mut Self {
        self.inner.is_added = is_added;
        self
    }
}

impl AsRef<ToggleBotIsAddedToAttachmentMenu> for ToggleBotIsAddedToAttachmentMenu {
    fn as_ref(&self) -> &ToggleBotIsAddedToAttachmentMenu {
        self
    }
}

impl AsRef<ToggleBotIsAddedToAttachmentMenu> for RTDToggleBotIsAddedToAttachmentMenuBuilder {
    fn as_ref(&self) -> &ToggleBotIsAddedToAttachmentMenu {
        &self.inner
    }
}
