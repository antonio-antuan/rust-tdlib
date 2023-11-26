use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Represents a button to be shown above inline query results
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct InlineQueryResultsButton {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The text of the button

    #[serde(default)]
    text: String,
    /// Type of the button

    #[serde(rename(serialize = "type", deserialize = "type"))]
    #[serde(skip_serializing_if = "InlineQueryResultsButtonType::_is_default")]
    type_: InlineQueryResultsButtonType,
}

impl RObject for InlineQueryResultsButton {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl InlineQueryResultsButton {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> InlineQueryResultsButtonBuilder {
        let mut inner = InlineQueryResultsButton::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        InlineQueryResultsButtonBuilder { inner }
    }

    pub fn text(&self) -> &String {
        &self.text
    }

    pub fn type_(&self) -> &InlineQueryResultsButtonType {
        &self.type_
    }
}

#[doc(hidden)]
pub struct InlineQueryResultsButtonBuilder {
    inner: InlineQueryResultsButton,
}

#[deprecated]
pub type RTDInlineQueryResultsButtonBuilder = InlineQueryResultsButtonBuilder;

impl InlineQueryResultsButtonBuilder {
    pub fn build(&self) -> InlineQueryResultsButton {
        self.inner.clone()
    }

    pub fn text<T: AsRef<str>>(&mut self, text: T) -> &mut Self {
        self.inner.text = text.as_ref().to_string();
        self
    }

    pub fn type_<T: AsRef<InlineQueryResultsButtonType>>(&mut self, type_: T) -> &mut Self {
        self.inner.type_ = type_.as_ref().clone();
        self
    }
}

impl AsRef<InlineQueryResultsButton> for InlineQueryResultsButton {
    fn as_ref(&self) -> &InlineQueryResultsButton {
        self
    }
}

impl AsRef<InlineQueryResultsButton> for InlineQueryResultsButtonBuilder {
    fn as_ref(&self) -> &InlineQueryResultsButton {
        &self.inner
    }
}
