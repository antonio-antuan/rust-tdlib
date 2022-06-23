use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns information about a bot that can be added to attachment menu
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetAttachmentMenuBot {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Bot's user identifier
    bot_user_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetAttachmentMenuBot {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetAttachmentMenuBot {}

impl GetAttachmentMenuBot {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetAttachmentMenuBotBuilder {
        let mut inner = GetAttachmentMenuBot::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getAttachmentMenuBot".to_string();

        RTDGetAttachmentMenuBotBuilder { inner }
    }

    pub fn bot_user_id(&self) -> i64 {
        self.bot_user_id
    }
}

#[doc(hidden)]
pub struct RTDGetAttachmentMenuBotBuilder {
    inner: GetAttachmentMenuBot,
}

impl RTDGetAttachmentMenuBotBuilder {
    pub fn build(&self) -> GetAttachmentMenuBot {
        self.inner.clone()
    }

    pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
        self.inner.bot_user_id = bot_user_id;
        self
    }
}

impl AsRef<GetAttachmentMenuBot> for GetAttachmentMenuBot {
    fn as_ref(&self) -> &GetAttachmentMenuBot {
        self
    }
}

impl AsRef<GetAttachmentMenuBot> for RTDGetAttachmentMenuBotBuilder {
    fn as_ref(&self) -> &GetAttachmentMenuBot {
        &self.inner
    }
}
