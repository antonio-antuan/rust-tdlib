use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Represents a date according to the Gregorian calendar
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Date {
    #[doc(hidden)]
    #[serde(rename(serialize = "@type", deserialize = "@type"))]
    td_name: String,
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Day of the month, 1-31
    day: i32,
    /// Month, 1-12
    month: i32,
    /// Year, 1-9999
    year: i32,
}

impl RObject for Date {
    #[doc(hidden)]
    fn td_name(&self) -> &'static str {
        "date"
    }
    #[doc(hidden)]
    fn extra(&self) -> Option<String> {
        self.extra.clone()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
    fn to_json(&self) -> RTDResult<String> {
        Ok(serde_json::to_string(self)?)
    }
}

impl Date {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDDateBuilder {
        let mut inner = Date::default();
        inner.td_name = "date".to_string();
        inner.extra = Some(Uuid::new_v4().to_string());
        inner.client_id = None;
        RTDDateBuilder { inner }
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
pub struct RTDDateBuilder {
    inner: Date,
}

impl RTDDateBuilder {
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

impl AsRef<Date> for RTDDateBuilder {
    fn as_ref(&self) -> &Date {
        &self.inner
    }
}
