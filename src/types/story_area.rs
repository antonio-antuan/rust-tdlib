use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a clickable rectangle area on a story media
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryArea {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Position of the area
    position: StoryAreaPosition,
    /// Type of the area

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "StoryAreaType::_is_default")]
    type_: StoryAreaType,
}

impl RObject for StoryArea {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl StoryArea {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StoryAreaBuilder {
        let mut inner = StoryArea::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StoryAreaBuilder { inner }
    }

    pub fn position(&self) -> &StoryAreaPosition {
        &self.position
    }

    pub fn type_(&self) -> &StoryAreaType {
        &self.type_
    }
}

#[doc(hidden)]
pub struct StoryAreaBuilder {
    inner: StoryArea,
}

#[deprecated]
pub type RTDStoryAreaBuilder = StoryAreaBuilder;

impl StoryAreaBuilder {
    pub fn build(&self) -> StoryArea {
        self.inner.clone()
    }

    pub fn position<T: AsRef<StoryAreaPosition>>(&mut self, position: T) -> &mut Self {
        self.inner.position = position.as_ref().clone();
        self
    }

    pub fn type_<T: AsRef<StoryAreaType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }
}

impl AsRef<StoryArea> for StoryArea {
    fn as_ref(&self) -> &StoryArea {
        self
    }
}

impl AsRef<StoryArea> for StoryAreaBuilder {
    fn as_ref(&self) -> &StoryArea {
        &self.inner
    }
}
