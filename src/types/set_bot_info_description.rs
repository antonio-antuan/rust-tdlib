use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sets the text shown in the chat with a bot if the chat is empty. Can be called only if userTypeBot.can_be_edited == true
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetBotInfoDescription {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the target bot

    #[serde(default)]
    bot_user_id: i64,
    /// A two-letter ISO 639-1 language code. If empty, the description will be shown to all users for whose languages there is no dedicated description

    #[serde(default)]
    language_code: String,
    /// Sets the text shown in the chat with a bot if the chat is empty. Can be called only if userTypeBot.can_be_edited == true

    #[serde(default)]
    description: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetBotInfoDescription {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetBotInfoDescription {}

impl SetBotInfoDescription {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetBotInfoDescriptionBuilder {
        let mut inner = SetBotInfoDescription::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setBotInfoDescription".to_string();

        SetBotInfoDescriptionBuilder { inner }
    }

    pub fn bot_user_id(&self) -> i64 {
        self.bot_user_id
    }

    pub fn language_code(&self) -> &String {
        &self.language_code
    }

    pub fn description(&self) -> &String {
        &self.description
    }
}

#[doc(hidden)]
pub struct SetBotInfoDescriptionBuilder {
    inner: SetBotInfoDescription,
}

#[deprecated]
pub type RTDSetBotInfoDescriptionBuilder = SetBotInfoDescriptionBuilder;

impl SetBotInfoDescriptionBuilder {
    pub fn build(&self) -> SetBotInfoDescription {
        self.inner.clone()
    }

    pub fn bot_user_id(&mut self, bot_user_id: i64) -> &mut Self {
        self.inner.bot_user_id = bot_user_id;
        self
    }

    pub fn language_code<T: AsRef<str>>(&mut self, language_code: T) -> &mut Self {
        self.inner.language_code = language_code.as_ref().to_string();
        self
    }

    pub fn description<T: AsRef<str>>(&mut self, description: T) -> &mut Self {
        self.inner.description = description.as_ref().to_string();
        self
    }
}

impl AsRef<SetBotInfoDescription> for SetBotInfoDescription {
    fn as_ref(&self) -> &SetBotInfoDescription {
        self
    }
}

impl AsRef<SetBotInfoDescription> for SetBotInfoDescriptionBuilder {
    fn as_ref(&self) -> &SetBotInfoDescription {
        &self.inner
    }
}
