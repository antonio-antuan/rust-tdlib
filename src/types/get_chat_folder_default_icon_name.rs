use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns default icon name for a folder. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetChatFolderDefaultIconName {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Chat folder
    folder: ChatFolder,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetChatFolderDefaultIconName {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetChatFolderDefaultIconName {}

impl GetChatFolderDefaultIconName {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetChatFolderDefaultIconNameBuilder {
        let mut inner = GetChatFolderDefaultIconName::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getChatFolderDefaultIconName".to_string();

        GetChatFolderDefaultIconNameBuilder { inner }
    }

    pub fn folder(&self) -> &ChatFolder {
        &self.folder
    }
}

#[doc(hidden)]
pub struct GetChatFolderDefaultIconNameBuilder {
    inner: GetChatFolderDefaultIconName,
}

#[deprecated]
pub type RTDGetChatFolderDefaultIconNameBuilder = GetChatFolderDefaultIconNameBuilder;

impl GetChatFolderDefaultIconNameBuilder {
    pub fn build(&self) -> GetChatFolderDefaultIconName {
        self.inner.clone()
    }

    pub fn folder<T: AsRef<ChatFolder>>(&mut self, folder: T) -> &mut Self {
        self.inner.folder = folder.as_ref().clone();
        self
    }
}

impl AsRef<GetChatFolderDefaultIconName> for GetChatFolderDefaultIconName {
    fn as_ref(&self) -> &GetChatFolderDefaultIconName {
        self
    }
}

impl AsRef<GetChatFolderDefaultIconName> for GetChatFolderDefaultIconNameBuilder {
    fn as_ref(&self) -> &GetChatFolderDefaultIconName {
        &self.inner
    }
}
