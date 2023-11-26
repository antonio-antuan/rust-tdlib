use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns invite links created by the current user for a shareable chat folder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatFolderInviteLinks {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat folder identifier

    #[serde(default)]
    chat_folder_id: i32,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatFolderInviteLinks {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatFolderInviteLinks {}

impl GetChatFolderInviteLinks {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatFolderInviteLinksBuilder {
        let mut inner = GetChatFolderInviteLinks::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatFolderInviteLinks".to_string();

        GetChatFolderInviteLinksBuilder { inner }
    }

    pub fn chat_folder_id(&self) -> i32 {
        self.chat_folder_id
    }
}

#[doc(hidden)]
pub struct GetChatFolderInviteLinksBuilder {
    inner: GetChatFolderInviteLinks,
}

#[deprecated]
pub type RTDGetChatFolderInviteLinksBuilder = GetChatFolderInviteLinksBuilder;

impl GetChatFolderInviteLinksBuilder {
    pub fn build(&self) -> GetChatFolderInviteLinks {
        self.inner.clone()
    }

    pub fn chat_folder_id(&mut self, chat_folder_id: i32) -> &mut Self {
        self.inner.chat_folder_id = chat_folder_id;
        self
    }
}

impl AsRef<GetChatFolderInviteLinks> for GetChatFolderInviteLinks {
    fn as_ref(&self) -> &GetChatFolderInviteLinks {
        self
    }
}

impl AsRef<GetChatFolderInviteLinks> for GetChatFolderInviteLinksBuilder {
    fn as_ref(&self) -> &GetChatFolderInviteLinks {
        &self.inner
    }
}
