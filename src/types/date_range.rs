use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a date range
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct DateRange {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Point in time (Unix timestamp) at which the date range begins

    #[serde(default)]
    start_date: i32,
    /// Point in time (Unix timestamp) at which the date range ends

    #[serde(default)]
    end_date: i32,
}

impl RObject for DateRange {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl DateRange {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DateRangeBuilder {
        let mut inner = DateRange::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        DateRangeBuilder { inner }
    }

    pub fn start_date(&self) -> i32 {
        self.start_date
    }

    pub fn end_date(&self) -> i32 {
        self.end_date
    }
}

#[doc(hidden)]
pub struct DateRangeBuilder {
    inner: DateRange,
}

#[deprecated]
pub type RTDDateRangeBuilder = DateRangeBuilder;

impl DateRangeBuilder {
    pub fn build(&self) -> DateRange {
        self.inner.clone()
    }

    pub fn start_date(&mut self, start_date: i32) -> &mut Self {
        self.inner.start_date = start_date;
        self
    }

    pub fn end_date(&mut self, end_date: i32) -> &mut Self {
        self.inner.end_date = end_date;
        self
    }
}

impl AsRef<DateRange> for DateRange {
    fn as_ref(&self) -> &DateRange {
        self
    }
}

impl AsRef<DateRange> for DateRangeBuilder {
    fn as_ref(&self) -> &DateRange {
        &self.inner
    }
}
