use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Contains a list of text entities
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TextEntities {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// List of text entities

    #[serde(default)]
    entities: Vec<TextEntity>,
}

impl RObject for TextEntities {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TextEntities {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> TextEntitiesBuilder {
        let mut inner = TextEntities::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        TextEntitiesBuilder { inner }
    }

    pub fn entities(&self) -> &Vec<TextEntity> {
        &self.entities
    }
}

#[doc(hidden)]
pub struct TextEntitiesBuilder {
    inner: TextEntities,
}

#[deprecated]
pub type RTDTextEntitiesBuilder = TextEntitiesBuilder;

impl TextEntitiesBuilder {
    pub fn build(&self) -> TextEntities {
        self.inner.clone()
    }

    pub fn entities(&mut self, entities: Vec<TextEntity>) -> &mut Self {
        self.inner.entities = entities;
        self
    }
}

impl AsRef<TextEntities> for TextEntities {
    fn as_ref(&self) -> &TextEntities {
        self
    }
}

impl AsRef<TextEntities> for TextEntitiesBuilder {
    fn as_ref(&self) -> &TextEntities {
        &self.inner
    }
}
