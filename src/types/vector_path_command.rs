use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Represents a vector path command
pub trait TDVectorPathCommand: Debug + RObject {}

/// Represents a vector path command
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum VectorPathCommand {
    #[doc(hidden)]
    _Default,
    /// A cubic Bézier curve to a given point
    #[serde(rename = "vectorPathCommandCubicBezierCurve")]
    CubicBezierCurve(VectorPathCommandCubicBezierCurve),
    /// A straight line to a given point
    #[serde(rename = "vectorPathCommandLine")]
    Line(VectorPathCommandLine),
}

impl Default for VectorPathCommand {
    fn default() -> Self {
        VectorPathCommand::_Default
    }
}

impl RObject for VectorPathCommand {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            VectorPathCommand::CubicBezierCurve(t) => t.extra(),
            VectorPathCommand::Line(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            VectorPathCommand::CubicBezierCurve(t) => t.client_id(),
            VectorPathCommand::Line(t) => t.client_id(),

            _ => None,
        }
    }
}

impl VectorPathCommand {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, VectorPathCommand::_Default)
    }
}

impl AsRef<VectorPathCommand> for VectorPathCommand {
    fn as_ref(&self) -> &VectorPathCommand {
        self
    }
}

/// A cubic Bézier curve to a given point
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VectorPathCommandCubicBezierCurve {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The start control point of the curve
    start_control_point: Point,
    /// The end control point of the curve
    end_control_point: Point,
    /// The end point of the curve
    end_point: Point,
}

impl RObject for VectorPathCommandCubicBezierCurve {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDVectorPathCommand for VectorPathCommandCubicBezierCurve {}

impl VectorPathCommandCubicBezierCurve {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> VectorPathCommandCubicBezierCurveBuilder {
        let mut inner = VectorPathCommandCubicBezierCurve::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        VectorPathCommandCubicBezierCurveBuilder { inner }
    }

    pub fn start_control_point(&self) -> &Point {
        &self.start_control_point
    }

    pub fn end_control_point(&self) -> &Point {
        &self.end_control_point
    }

    pub fn end_point(&self) -> &Point {
        &self.end_point
    }
}

#[doc(hidden)]
pub struct VectorPathCommandCubicBezierCurveBuilder {
    inner: VectorPathCommandCubicBezierCurve,
}

#[deprecated]
pub type RTDVectorPathCommandCubicBezierCurveBuilder = VectorPathCommandCubicBezierCurveBuilder;

impl VectorPathCommandCubicBezierCurveBuilder {
    pub fn build(&self) -> VectorPathCommandCubicBezierCurve {
        self.inner.clone()
    }

    pub fn start_control_point<T: AsRef<Point>>(&mut self, start_control_point: T) -> &mut Self {
        self.inner.start_control_point = start_control_point.as_ref().clone();
        self
    }

    pub fn end_control_point<T: AsRef<Point>>(&mut self, end_control_point: T) -> &mut Self {
        self.inner.end_control_point = end_control_point.as_ref().clone();
        self
    }

    pub fn end_point<T: AsRef<Point>>(&mut self, end_point: T) -> &mut Self {
        self.inner.end_point = end_point.as_ref().clone();
        self
    }
}

impl AsRef<VectorPathCommandCubicBezierCurve> for VectorPathCommandCubicBezierCurve {
    fn as_ref(&self) -> &VectorPathCommandCubicBezierCurve {
        self
    }
}

impl AsRef<VectorPathCommandCubicBezierCurve> for VectorPathCommandCubicBezierCurveBuilder {
    fn as_ref(&self) -> &VectorPathCommandCubicBezierCurve {
        &self.inner
    }
}

/// A straight line to a given point
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct VectorPathCommandLine {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The end point of the straight line
    end_point: Point,
}

impl RObject for VectorPathCommandLine {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDVectorPathCommand for VectorPathCommandLine {}

impl VectorPathCommandLine {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> VectorPathCommandLineBuilder {
        let mut inner = VectorPathCommandLine::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        VectorPathCommandLineBuilder { inner }
    }

    pub fn end_point(&self) -> &Point {
        &self.end_point
    }
}

#[doc(hidden)]
pub struct VectorPathCommandLineBuilder {
    inner: VectorPathCommandLine,
}

#[deprecated]
pub type RTDVectorPathCommandLineBuilder = VectorPathCommandLineBuilder;

impl VectorPathCommandLineBuilder {
    pub fn build(&self) -> VectorPathCommandLine {
        self.inner.clone()
    }

    pub fn end_point<T: AsRef<Point>>(&mut self, end_point: T) -> &mut Self {
        self.inner.end_point = end_point.as_ref().clone();
        self
    }
}

impl AsRef<VectorPathCommandLine> for VectorPathCommandLine {
    fn as_ref(&self) -> &VectorPathCommandLine {
        self
    }
}

impl AsRef<VectorPathCommandLine> for VectorPathCommandLineBuilder {
    fn as_ref(&self) -> &VectorPathCommandLine {
        &self.inner
    }
}
