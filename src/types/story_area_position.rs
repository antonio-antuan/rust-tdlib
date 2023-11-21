use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Describes position of a clickable rectangle area on a story media
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct StoryAreaPosition {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The abscissa of the rectangle's center, as a percentage of the media width

    #[serde(default)]
    x_percentage: f32,
    /// The ordinate of the rectangle's center, as a percentage of the media height

    #[serde(default)]
    y_percentage: f32,
    /// The width of the rectangle, as a percentage of the media width

    #[serde(default)]
    width_percentage: f32,
    /// The height of the rectangle, as a percentage of the media height

    #[serde(default)]
    height_percentage: f32,
    /// Clockwise rotation angle of the rectangle, in degrees; 0-360

    #[serde(default)]
    rotation_angle: f32,
}

impl RObject for StoryAreaPosition {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl StoryAreaPosition {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> StoryAreaPositionBuilder {
        let mut inner = StoryAreaPosition::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        StoryAreaPositionBuilder { inner }
    }

    pub fn x_percentage(&self) -> f32 {
        self.x_percentage
    }

    pub fn y_percentage(&self) -> f32 {
        self.y_percentage
    }

    pub fn width_percentage(&self) -> f32 {
        self.width_percentage
    }

    pub fn height_percentage(&self) -> f32 {
        self.height_percentage
    }

    pub fn rotation_angle(&self) -> f32 {
        self.rotation_angle
    }
}

#[doc(hidden)]
pub struct StoryAreaPositionBuilder {
    inner: StoryAreaPosition,
}

#[deprecated]
pub type RTDStoryAreaPositionBuilder = StoryAreaPositionBuilder;

impl StoryAreaPositionBuilder {
    pub fn build(&self) -> StoryAreaPosition {
        self.inner.clone()
    }

    pub fn x_percentage(&mut self, x_percentage: f32) -> &mut Self {
        self.inner.x_percentage = x_percentage;
        self
    }

    pub fn y_percentage(&mut self, y_percentage: f32) -> &mut Self {
        self.inner.y_percentage = y_percentage;
        self
    }

    pub fn width_percentage(&mut self, width_percentage: f32) -> &mut Self {
        self.inner.width_percentage = width_percentage;
        self
    }

    pub fn height_percentage(&mut self, height_percentage: f32) -> &mut Self {
        self.inner.height_percentage = height_percentage;
        self
    }

    pub fn rotation_angle(&mut self, rotation_angle: f32) -> &mut Self {
        self.inner.rotation_angle = rotation_angle;
        self
    }
}

impl AsRef<StoryAreaPosition> for StoryAreaPosition {
    fn as_ref(&self) -> &StoryAreaPosition {
        self
    }
}

impl AsRef<StoryAreaPosition> for StoryAreaPositionBuilder {
    fn as_ref(&self) -> &StoryAreaPosition {
        &self.inner
    }
}
