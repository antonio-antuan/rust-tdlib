use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains basic information about a chat folder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatFolderInfo {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Unique chat folder identifier

    #[serde(default)]
    id: i32,
    /// The title of the folder; 1-12 characters without line feeds

    #[serde(default)]
    title: String,
    /// The chosen or default icon for the chat folder
    icon: ChatFolderIcon,
    /// True, if at least one link has been created for the folder

    #[serde(default)]
    is_shareable: bool,
    /// True, if the chat folder has invite links created by the current user

    #[serde(default)]
    has_my_invite_links: bool,
}

impl RObject for ChatFolderInfo {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatFolderInfo {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatFolderInfoBuilder {
        let mut inner = ChatFolderInfo::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatFolderInfoBuilder { inner }
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn title(&self) -> &String {
        &self.title
    }

    pub fn icon(&self) -> &ChatFolderIcon {
        &self.icon
    }

    pub fn is_shareable(&self) -> bool {
        self.is_shareable
    }

    pub fn has_my_invite_links(&self) -> bool {
        self.has_my_invite_links
    }
}

#[doc(hidden)]
pub struct ChatFolderInfoBuilder {
    inner: ChatFolderInfo,
}

#[deprecated]
pub type RTDChatFolderInfoBuilder = ChatFolderInfoBuilder;

impl ChatFolderInfoBuilder {
    pub fn build(&self) -> ChatFolderInfo {
        self.inner.clone()
    }

    pub fn id(&mut self, id: i32) -> &mut Self {
        self.inner.id = id;
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }

    pub fn icon<T: AsRef<ChatFolderIcon>>(&mut self, icon: T) -> &mut Self {
        self.inner.icon = icon.as_ref().clone();
        self
    }

    pub fn is_shareable(&mut self, is_shareable: bool) -> &mut Self {
        self.inner.is_shareable = is_shareable;
        self
    }

    pub fn has_my_invite_links(&mut self, has_my_invite_links: bool) -> &mut Self {
        self.inner.has_my_invite_links = has_my_invite_links;
        self
    }
}

impl AsRef<ChatFolderInfo> for ChatFolderInfo {
    fn as_ref(&self) -> &ChatFolderInfo {
        self
    }
}

impl AsRef<ChatFolderInfo> for ChatFolderInfoBuilder {
    fn as_ref(&self) -> &ChatFolderInfo {
        &self.inner
    }
}
