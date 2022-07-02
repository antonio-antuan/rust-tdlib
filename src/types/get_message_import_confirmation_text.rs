use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns a confirmation text to be shown to the user before starting message import
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetMessageImportConfirmationText {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of a chat to which the messages will be imported. It must be an identifier of a private chat with a mutual contact or an identifier of a supergroup chat with can_change_info administrator right

    #[serde(default)]
    chat_id: i64,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetMessageImportConfirmationText {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetMessageImportConfirmationText {}

impl GetMessageImportConfirmationText {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetMessageImportConfirmationTextBuilder {
        let mut inner = GetMessageImportConfirmationText::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getMessageImportConfirmationText".to_string();

        GetMessageImportConfirmationTextBuilder { inner }
    }

    pub fn chat_id(&self) -> i64 {
        self.chat_id
    }
}

#[doc(hidden)]
pub struct GetMessageImportConfirmationTextBuilder {
    inner: GetMessageImportConfirmationText,
}

#[deprecated]
pub type RTDGetMessageImportConfirmationTextBuilder = GetMessageImportConfirmationTextBuilder;

impl GetMessageImportConfirmationTextBuilder {
    pub fn build(&self) -> GetMessageImportConfirmationText {
        self.inner.clone()
    }

    pub fn chat_id(&mut self, chat_id: i64) -> &mut Self {
        self.inner.chat_id = chat_id;
        self
    }
}

impl AsRef<GetMessageImportConfirmationText> for GetMessageImportConfirmationText {
    fn as_ref(&self) -> &GetMessageImportConfirmationText {
        self
    }
}

impl AsRef<GetMessageImportConfirmationText> for GetMessageImportConfirmationTextBuilder {
    fn as_ref(&self) -> &GetMessageImportConfirmationText {
        &self.inner
    }
}
