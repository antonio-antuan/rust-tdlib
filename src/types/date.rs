use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a date according to the Gregorian calendar
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Date {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Day of the month; 1-31

    #[serde(default)]
    day: i32,
    /// Month; 1-12

    #[serde(default)]
    month: i32,
    /// Year; 1-9999

    #[serde(default)]
    year: i32,
}

impl RObject for Date {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl Date {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> DateBuilder {
        let mut inner = Date::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        DateBuilder { inner }
    }

    pub fn day(&self) -> i32 {
        self.day
    }

    pub fn month(&self) -> i32 {
        self.month
    }

    pub fn year(&self) -> i32 {
        self.year
    }
}

#[doc(hidden)]
pub struct DateBuilder {
    inner: Date,
}

#[deprecated]
pub type RTDDateBuilder = DateBuilder;

impl DateBuilder {
    pub fn build(&self) -> Date {
        self.inner.clone()
    }

    pub fn day(&mut self, day: i32) -> &mut Self {
        self.inner.day = day;
        self
    }

    pub fn month(&mut self, month: i32) -> &mut Self {
        self.inner.month = month;
        self
    }

    pub fn year(&mut self, year: i32) -> &mut Self {
        self.inner.year = year;
        self
    }
}

impl AsRef<Date> for Date {
    fn as_ref(&self) -> &Date {
        self
    }
}

impl AsRef<Date> for DateBuilder {
    fn as_ref(&self) -> &Date {
        &self.inner
    }
}
