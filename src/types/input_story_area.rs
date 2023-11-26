use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes a clickable rectangle area on a story media to be added
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InputStoryArea {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Position of the area
    position: StoryAreaPosition,
    /// Type of the area

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "InputStoryAreaType::_is_default")]
    type_: InputStoryAreaType,
}

impl RObject for InputStoryArea {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl InputStoryArea {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InputStoryAreaBuilder {
        let mut inner = InputStoryArea::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InputStoryAreaBuilder { inner }
    }

    pub fn position(&self) -> &StoryAreaPosition {
        &self.position
    }

    pub fn type_(&self) -> &InputStoryAreaType {
        &self.type_
    }
}

#[doc(hidden)]
pub struct InputStoryAreaBuilder {
    inner: InputStoryArea,
}

#[deprecated]
pub type RTDInputStoryAreaBuilder = InputStoryAreaBuilder;

impl InputStoryAreaBuilder {
    pub fn build(&self) -> InputStoryArea {
        self.inner.clone()
    }

    pub fn position<T: AsRef<StoryAreaPosition>>(&mut self, position: T) -> &mut Self {
        self.inner.position = position.as_ref().clone();
        self
    }

    pub fn type_<T: AsRef<InputStoryAreaType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }
}

impl AsRef<InputStoryArea> for InputStoryArea {
    fn as_ref(&self) -> &InputStoryArea {
        self
    }
}

impl AsRef<InputStoryArea> for InputStoryAreaBuilder {
    fn as_ref(&self) -> &InputStoryArea {
        &self.inner
    }
}
