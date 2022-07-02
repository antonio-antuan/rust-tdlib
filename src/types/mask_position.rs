use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Position on a photo where a mask is placed
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MaskPosition {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Part of the face, relative to which the mask is placed

    #[serde(skip_serializing_if = "MaskPoint::_is_default")]
    point: MaskPoint,
    /// Shift by X-axis measured in widths of the mask scaled to the face size, from left to right. (For example, 1.0 will place the mask just to the left of the default mask position)

    #[serde(default)]
    x_shift: f32,
    /// Shift by Y-axis measured in heights of the mask scaled to the face size, from top to bottom. (For example, 1.0 will place the mask just below the default mask position)

    #[serde(default)]
    y_shift: f32,
    /// Mask scaling coefficient. (For example, 2.0 means a doubled size)

    #[serde(default)]
    scale: f32,
}

impl RObject for MaskPosition {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl MaskPosition {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> MaskPositionBuilder {
        let mut inner = MaskPosition::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        MaskPositionBuilder { inner }
    }

    pub fn point(&self) -> &MaskPoint {
        &self.point
    }

    pub fn x_shift(&self) -> f32 {
        self.x_shift
    }

    pub fn y_shift(&self) -> f32 {
        self.y_shift
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }
}

#[doc(hidden)]
pub struct MaskPositionBuilder {
    inner: MaskPosition,
}

#[deprecated]
pub type RTDMaskPositionBuilder = MaskPositionBuilder;

impl MaskPositionBuilder {
    pub fn build(&self) -> MaskPosition {
        self.inner.clone()
    }

    pub fn point<T: AsRef<MaskPoint>>(&mut self, point: T) -> &mut Self {
        self.inner.point = point.as_ref().clone();
        self
    }

    pub fn x_shift(&mut self, x_shift: f32) -> &mut Self {
        self.inner.x_shift = x_shift;
        self
    }

    pub fn y_shift(&mut self, y_shift: f32) -> &mut Self {
        self.inner.y_shift = y_shift;
        self
    }

    pub fn scale(&mut self, scale: f32) -> &mut Self {
        self.inner.scale = scale;
        self
    }
}

impl AsRef<MaskPosition> for MaskPosition {
    fn as_ref(&self) -> &MaskPosition {
        self
    }
}

impl AsRef<MaskPosition> for MaskPositionBuilder {
    fn as_ref(&self) -> &MaskPosition {
        &self.inner
    }
}
