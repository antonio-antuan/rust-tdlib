use crate::errors::*;
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDTextEntitiesBuilder {
        let mut inner = TextEntities::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDTextEntitiesBuilder { inner }
    }

    pub fn entities(&self) -> &Vec<TextEntity> {
        &self.entities
    }
}

#[doc(hidden)]
pub struct RTDTextEntitiesBuilder {
    inner: TextEntities,
}

impl RTDTextEntitiesBuilder {
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

impl AsRef<TextEntities> for RTDTextEntitiesBuilder {
    fn as_ref(&self) -> &TextEntities {
        &self.inner
    }
}
