use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents an icon for a chat folder
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ChatFolderIcon {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The chosen icon name for short folder representation; one of "All", "Unread", "Unmuted", "Bots", "Channels", "Groups", "Private", "Custom", "Setup", "Cat", "Crown", "Favorite", "Flower", "Game", "Home", "Love", "Mask", "Party", "Sport", "Study", "Trade", "Travel", "Work", "Airplane", "Book", "Light", "Like", "Money", "Note", "Palette"

    #[serde(default)]
    name: String,
}

impl RObject for ChatFolderIcon {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl ChatFolderIcon {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> ChatFolderIconBuilder {
        let mut inner = ChatFolderIcon::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        ChatFolderIconBuilder { inner }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[doc(hidden)]
pub struct ChatFolderIconBuilder {
    inner: ChatFolderIcon,
}

#[deprecated]
pub type RTDChatFolderIconBuilder = ChatFolderIconBuilder;

impl ChatFolderIconBuilder {
    pub fn build(&self) -> ChatFolderIcon {
        self.inner.clone()
    }

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }
}

impl AsRef<ChatFolderIcon> for ChatFolderIcon {
    fn as_ref(&self) -> &ChatFolderIcon {
        self
    }
}

impl AsRef<ChatFolderIcon> for ChatFolderIconBuilder {
    fn as_ref(&self) -> &ChatFolderIcon {
        &self.inner
    }
}
