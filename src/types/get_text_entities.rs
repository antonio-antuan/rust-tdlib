use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Returns all entities (mentions, hashtags, cashtags, bot commands, bank card numbers, URLs, and email addresses) contained in the text. Can be called synchronously
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetTextEntities {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The text in which to look for entites

    #[serde(default)]
    text: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetTextEntities {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetTextEntities {}

impl GetTextEntities {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> GetTextEntitiesBuilder {
        let mut inner = GetTextEntities::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getTextEntities".to_string();

        GetTextEntitiesBuilder { inner }
    }

    pub fn text(&self) -> &String {
        &self.text
    }
}

#[doc(hidden)]
pub struct GetTextEntitiesBuilder {
    inner: GetTextEntities,
}

#[deprecated]
pub type RTDGetTextEntitiesBuilder = GetTextEntitiesBuilder;

impl GetTextEntitiesBuilder {
    pub fn build(&self) -> GetTextEntities {
        self.inner.clone()
    }

    pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().to_string();
        self
    }
}

impl AsRef<GetTextEntities> for GetTextEntities {
    fn as_ref(&self) -> &GetTextEntities {
        self
    }
}

impl AsRef<GetTextEntities> for GetTextEntitiesBuilder {
    fn as_ref(&self) -> &GetTextEntities {
        &self.inner
    }
}
