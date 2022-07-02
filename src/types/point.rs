use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// A point on a Cartesian plane
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Point {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The point's first coordinate

    #[serde(default)]
    x: f32,
    /// The point's second coordinate

    #[serde(default)]
    y: f32,
}

impl RObject for Point {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Point {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PointBuilder {
        let mut inner = Point::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PointBuilder { inner }
    }

    pub fn x(&self) -> f32 {
        self.x
    }

    pub fn y(&self) -> f32 {
        self.y
    }
}

#[doc(hidden)]
pub struct PointBuilder {
    inner: Point,
}

#[deprecated]
pub type RTDPointBuilder = PointBuilder;

impl PointBuilder {
    pub fn build(&self) -> Point {
        self.inner.clone()
    }

    pub fn x(&mut self, x: f32) -> &mut Self {
        self.inner.x = x;
        self
    }

    pub fn y(&mut self, y: f32) -> &mut Self {
        self.inner.y = y;
        self
    }
}

impl AsRef<Point> for Point {
    fn as_ref(&self) -> &Point {
        self
    }
}

impl AsRef<Point> for PointBuilder {
    fn as_ref(&self) -> &Point {
        &self.inner
    }
}
