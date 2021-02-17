use crate::errors::*;
use crate::types::*;
use uuid::Uuid;

/// Returns the default text for invitation messages to be used as a placeholder when the current user invites friends to Telegram
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetInviteText {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for GetInviteText {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for GetInviteText {}

impl GetInviteText {
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDGetInviteTextBuilder {
        let mut inner = GetInviteText::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "getInviteText".to_string();

        RTDGetInviteTextBuilder { inner }
    }
}

#[doc(hidden)]
pub struct RTDGetInviteTextBuilder {
    inner: GetInviteText,
}

impl RTDGetInviteTextBuilder {
    pub fn build(&self) -> GetInviteText {
        self.inner.clone()
    }
}

impl AsRef<GetInviteText> for GetInviteText {
    fn as_ref(&self) -> &GetInviteText {
        self
    }
}

impl AsRef<GetInviteText> for RTDGetInviteTextBuilder {
    fn as_ref(&self) -> &GetInviteText {
        &self.inner
    }
}
