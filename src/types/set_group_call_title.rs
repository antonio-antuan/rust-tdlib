use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Sets group call title. Requires groupCall.can_be_managed group call flag
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SetGroupCallTitle {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Group call identifier

    #[serde(default)]
    group_call_id: i32,
    /// New group call title; 1-64 characters

    #[serde(default)]
    title: String,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for SetGroupCallTitle {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for SetGroupCallTitle {}

impl SetGroupCallTitle {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> SetGroupCallTitleBuilder {
        let mut inner = SetGroupCallTitle::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "setGroupCallTitle".to_string();

        SetGroupCallTitleBuilder { inner }
    }

    pub fn group_call_id(&self) -> i32 {
        self.group_call_id
    }

    pub fn title(&self) -> &String {
        &self.title
    }
}

#[doc(hidden)]
pub struct SetGroupCallTitleBuilder {
    inner: SetGroupCallTitle,
}

#[deprecated]
pub type RTDSetGroupCallTitleBuilder = SetGroupCallTitleBuilder;

impl SetGroupCallTitleBuilder {
    pub fn build(&self) -> SetGroupCallTitle {
        self.inner.clone()
    }

    pub fn group_call_id(&mut self, group_call_id: i32) -> &mut Self {
        self.inner.group_call_id = group_call_id;
        self
    }

    pub fn title<T: AsRef<str>>(&mut self, title: T) -> &mut Self {
        self.inner.title = title.as_ref().to_string();
        self
    }
}

impl AsRef<SetGroupCallTitle> for SetGroupCallTitle {
    fn as_ref(&self) -> &SetGroupCallTitle {
        self
    }
}

impl AsRef<SetGroupCallTitle> for SetGroupCallTitleBuilder {
    fn as_ref(&self) -> &SetGroupCallTitle {
        &self.inner
    }
}
