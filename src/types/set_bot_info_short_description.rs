use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sets the text shown on a bot's profile page and sent together with the link when users share the bot. Can be called only if userTypeBot.can_be_edited == true
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetBotInfoShortDescription {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the target bot

    #[serde(default)]
    bot_user_id: i64,
    /// A two-letter ISO 639-1 language code. If empty, the short description will be shown to all users for whose languages there is no dedicated description

    #[serde(default)]
    language_code: String,
    /// New bot's short description on the specified language

    #[serde(default)]
    short_description: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetBotInfoShortDescription {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetBotInfoShortDescription {}

impl SetBotInfoShortDescription {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetBotInfoShortDescriptionBuilder {
        let mut inner = SetBotInfoShortDescription::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setBotInfoShortDescription".to_string();

        SetBotInfoShortDescriptionBuilder { inner }
    }

    pub fn bot_user_id(&self) -> i64 {
        self.bot_user_id
    }

    pub fn language_code(&self) -> &String {
        &self.language_code
    }

    pub fn short_description(&self) -> &String {
        &self.short_description
    }
}

#[doc(hidden)]
pub struct SetBotInfoShortDescriptionBuilder {
    inner: SetBotInfoShortDescription,
}

#[deprecated]
pub type RTDSetBotInfoShortDescriptionBuilder = SetBotInfoShortDescriptionBuilder;

impl SetBotInfoShortDescriptionBuilder {
    pub fn build(&self) -> SetBotInfoShortDescription {
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

    pub fn short_description<T: AsRef<str>>(&mut self, short_description: T) -> &mut Self {
        self.inner.short_description = short_description.as_ref().to_string();
        self
    }
}

impl AsRef<SetBotInfoShortDescription> for SetBotInfoShortDescription {
    fn as_ref(&self) -> &SetBotInfoShortDescription {
        self
    }
}

impl AsRef<SetBotInfoShortDescription> for SetBotInfoShortDescriptionBuilder {
    fn as_ref(&self) -> &SetBotInfoShortDescription {
        &self.inner
    }
}
