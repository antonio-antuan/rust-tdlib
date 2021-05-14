use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Creates a new supergroup from an existing basic group and sends a corresponding messageChatUpgradeTo and messageChatUpgradeFrom; requires creator privileges. Deactivates the original basic group
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UpgradeBasicGroupChatToSupergroupChat {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the chat to upgrade
    chat_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for UpgradeBasicGroupChatToSupergroupChat {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for UpgradeBasicGroupChatToSupergroupChat {}

impl UpgradeBasicGroupChatToSupergroupChat {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDUpgradeBasicGroupChatToSupergroupChatBuilder {
        let mut inner = UpgradeBasicGroupChatToSupergroupChat::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "upgradeBasicGroupChatToSupergroupChat".to_string();

        RTDUpgradeBasicGroupChatToSupergroupChatBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct RTDUpgradeBasicGroupChatToSupergroupChatBuilder {
    inner: UpgradeBasicGroupChatToSupergroupChat,
}

impl RTDUpgradeBasicGroupChatToSupergroupChatBuilder {
    pub fn build(&self) -> UpgradeBasicGroupChatToSupergroupChat {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<UpgradeBasicGroupChatToSupergroupChat> for UpgradeBasicGroupChatToSupergroupChat {
    fn as_ref(&self) -> &UpgradeBasicGroupChatToSupergroupChat {
        self
    }
}

impl AsRef<UpgradeBasicGroupChatToSupergroupChat>
    for RTDUpgradeBasicGroupChatToSupergroupChatBuilder
{
    fn as_ref(&self) -> &UpgradeBasicGroupChatToSupergroupChat {
        &self.inner
    }
}
