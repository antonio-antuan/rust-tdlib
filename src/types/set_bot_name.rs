use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sets the name of a bot. Can be called only if userTypeBot.can_be_edited == true
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetBotName {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identifier of the target bot

    #[serde(default)]
    bot_user_id: i64,
    /// A two-letter ISO 639-1 language code. If empty, the name will be shown to all users for whose languages there is no dedicated name

    #[serde(default)]
    language_code: String,
    /// New bot's name on the specified language; 0-64 characters; must be non-empty if language code is empty

    #[serde(default)]
    name: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetBotName {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetBotName {}

impl SetBotName {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetBotNameBuilder {
        let mut inner = SetBotName::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setBotName".to_string();

        SetBotNameBuilder { inner }
    }

    pub fn bot_user_id(&self) -> i64 {
        self.bot_user_id
    }

    pub fn language_code(&self) -> &String {
        &self.language_code
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}

#[doc(hidden)]
pub struct SetBotNameBuilder {
    inner: SetBotName,
}

#[deprecated]
pub type RTDSetBotNameBuilder = SetBotNameBuilder;

impl SetBotNameBuilder {
    pub fn build(&self) -> SetBotName {
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

    pub fn name<T: AsRef<str>>(&mut self, name: T) -> &mut Self {
        self.inner.name = name.as_ref().to_string();
        self
    }
}

impl AsRef<SetBotName> for SetBotName {
    fn as_ref(&self) -> &SetBotName {
        self
    }
}

impl AsRef<SetBotName> for SetBotNameBuilder {
    fn as_ref(&self) -> &SetBotName {
        &self.inner
    }
}
