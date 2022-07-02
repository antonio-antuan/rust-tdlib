use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

/// Adds a user to the contact list or edits an existing contact by their user identifier
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AddContact {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// The contact to add or edit; phone number can be empty and needs to be specified only if known, vCard is ignored
    contact: Contact,
    /// True, if the new contact needs to be allowed to see current user's phone number. A corresponding rule to userPrivacySettingShowPhoneNumber will be added if needed. Use the field userFullInfo.need_phone_number_privacy_exception to check whether the current user needs to be asked to share their phone number

    #[serde(default)]
    share_phone_number: bool,

    #[serde(rename(serialize = "@type"))]
    td_type: String,
}

impl RObject for AddContact {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl RFunction for AddContact {}

impl AddContact {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> AddContactBuilder {
        let mut inner = AddContact::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        inner.td_type = "addContact".to_string();

        AddContactBuilder { inner }
    }

    pub fn contact(&self) -> &Contact {
        &self.contact
    }

    pub fn share_phone_number(&self) -> bool {
        self.share_phone_number
    }
}

#[doc(hidden)]
pub struct AddContactBuilder {
    inner: AddContact,
}

#[deprecated]
pub type RTDAddContactBuilder = AddContactBuilder;

impl AddContactBuilder {
    pub fn build(&self) -> AddContact {
        self.inner.clone()
    }

    pub fn contact<T: AsRef<Contact>>(&mut self, contact: T) -> &mut Self {
        self.inner.contact = contact.as_ref().clone();
        self
    }

    pub fn share_phone_number(&mut self, share_phone_number: bool) -> &mut Self {
        self.inner.share_phone_number = share_phone_number;
        self
    }
}

impl AsRef<AddContact> for AddContact {
    fn as_ref(&self) -> &AddContact {
        self
    }
}

impl AsRef<AddContact> for AddContactBuilder {
    fn as_ref(&self) -> &AddContact {
        &self.inner
    }
}
