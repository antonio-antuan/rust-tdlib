use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a list of emoji
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Emojis {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of emojis

    #[serde(default)]
    emojis: Vec<String>,
}

impl RObject for Emojis {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Emojis {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> EmojisBuilder {
        let mut inner = Emojis::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        EmojisBuilder { inner }
    }

    pub fn emojis(&self) -> &Vec<String> {
        &self.emojis
    }
}

#[doc(hidden)]
pub struct EmojisBuilder {
    inner: Emojis,
}

#[deprecated]
pub type RTDEmojisBuilder = EmojisBuilder;

impl EmojisBuilder {
    pub fn build(&self) -> Emojis {
        self.inner.clone()
    }

    pub fn emojis(&mut self, emojis: Vec<String>) -> &mut Self {
        self.inner.emojis = emojis;
        self
    }
}

impl AsRef<Emojis> for Emojis {
    fn as_ref(&self) -> &Emojis {
        self
    }
}

impl AsRef<Emojis> for EmojisBuilder {
    fn as_ref(&self) -> &Emojis {
        &self.inner
    }
}
