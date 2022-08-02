use crate::errors::Result;
use crate::types::*;
use uuid::Uuid;

use std::fmt::Debug;

/// Contains information about a Telegram Passport element
pub trait TDPassportElement: Debug + RObject {}

/// Contains information about a Telegram Passport element
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "@type")]
pub enum PassportElement {
    #[doc(hidden)]
    _Default,
    /// Returns one of the available Telegram Passport elements
    #[serde(rename = "getPassportElement")]
    GetPassportElement(GetPassportElement),
    /// A Telegram Passport element containing the user's address
    #[serde(rename = "passportElementAddress")]
    Address(PassportElementAddress),
    /// A Telegram Passport element containing the user's bank statement
    #[serde(rename = "passportElementBankStatement")]
    BankStatement(PassportElementBankStatement),
    /// A Telegram Passport element containing the user's driver license
    #[serde(rename = "passportElementDriverLicense")]
    DriverLicense(PassportElementDriverLicense),
    /// A Telegram Passport element containing the user's email address
    #[serde(rename = "passportElementEmailAddress")]
    EmailAddress(PassportElementEmailAddress),
    /// A Telegram Passport element containing the user's identity card
    #[serde(rename = "passportElementIdentityCard")]
    IdentityCard(PassportElementIdentityCard),
    /// A Telegram Passport element containing the user's internal passport
    #[serde(rename = "passportElementInternalPassport")]
    InternalPassport(PassportElementInternalPassport),
    /// A Telegram Passport element containing the user's passport
    #[serde(rename = "passportElementPassport")]
    Passport(PassportElementPassport),
    /// A Telegram Passport element containing the user's passport registration pages
    #[serde(rename = "passportElementPassportRegistration")]
    PassportRegistration(PassportElementPassportRegistration),
    /// A Telegram Passport element containing the user's personal details
    #[serde(rename = "passportElementPersonalDetails")]
    PersonalDetails(PassportElementPersonalDetails),
    /// A Telegram Passport element containing the user's phone number
    #[serde(rename = "passportElementPhoneNumber")]
    PhoneNumber(PassportElementPhoneNumber),
    /// A Telegram Passport element containing the user's rental agreement
    #[serde(rename = "passportElementRentalAgreement")]
    RentalAgreement(PassportElementRentalAgreement),
    /// A Telegram Passport element containing the user's temporary registration
    #[serde(rename = "passportElementTemporaryRegistration")]
    TemporaryRegistration(PassportElementTemporaryRegistration),
    /// A Telegram Passport element containing the user's utility bill
    #[serde(rename = "passportElementUtilityBill")]
    UtilityBill(PassportElementUtilityBill),
    /// Adds an element to the user's Telegram Passport. May return an error with a message "PHONE_VERIFICATION_NEEDED" or "EMAIL_VERIFICATION_NEEDED" if the chosen phone number or the chosen email address must be verified first
    #[serde(rename = "setPassportElement")]
    SetPassportElement(SetPassportElement),
}

impl Default for PassportElement {
    fn default() -> Self {
        PassportElement::_Default
    }
}

impl RObject for PassportElement {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        match self {
            PassportElement::GetPassportElement(t) => t.extra(),
            PassportElement::Address(t) => t.extra(),
            PassportElement::BankStatement(t) => t.extra(),
            PassportElement::DriverLicense(t) => t.extra(),
            PassportElement::EmailAddress(t) => t.extra(),
            PassportElement::IdentityCard(t) => t.extra(),
            PassportElement::InternalPassport(t) => t.extra(),
            PassportElement::Passport(t) => t.extra(),
            PassportElement::PassportRegistration(t) => t.extra(),
            PassportElement::PersonalDetails(t) => t.extra(),
            PassportElement::PhoneNumber(t) => t.extra(),
            PassportElement::RentalAgreement(t) => t.extra(),
            PassportElement::TemporaryRegistration(t) => t.extra(),
            PassportElement::UtilityBill(t) => t.extra(),
            PassportElement::SetPassportElement(t) => t.extra(),

            _ => None,
        }
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        match self {
            PassportElement::GetPassportElement(t) => t.client_id(),
            PassportElement::Address(t) => t.client_id(),
            PassportElement::BankStatement(t) => t.client_id(),
            PassportElement::DriverLicense(t) => t.client_id(),
            PassportElement::EmailAddress(t) => t.client_id(),
            PassportElement::IdentityCard(t) => t.client_id(),
            PassportElement::InternalPassport(t) => t.client_id(),
            PassportElement::Passport(t) => t.client_id(),
            PassportElement::PassportRegistration(t) => t.client_id(),
            PassportElement::PersonalDetails(t) => t.client_id(),
            PassportElement::PhoneNumber(t) => t.client_id(),
            PassportElement::RentalAgreement(t) => t.client_id(),
            PassportElement::TemporaryRegistration(t) => t.client_id(),
            PassportElement::UtilityBill(t) => t.client_id(),
            PassportElement::SetPassportElement(t) => t.client_id(),

            _ => None,
        }
    }
}

impl PassportElement {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    #[doc(hidden)]
    pub fn _is_default(&self) -> bool {
        matches!(self, PassportElement::_Default)
    }
}

impl AsRef<PassportElement> for PassportElement {
    fn as_ref(&self) -> &PassportElement {
        self
    }
}

/// A Telegram Passport element containing the user's address
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementAddress {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Address
    address: Address,
}

impl RObject for PassportElementAddress {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElement for PassportElementAddress {}

impl PassportElementAddress {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportElementAddressBuilder {
        let mut inner = PassportElementAddress::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportElementAddressBuilder { inner }
    }

    pub fn address(&self) -> &Address {
        &self.address
    }
}

#[doc(hidden)]
pub struct PassportElementAddressBuilder {
    inner: PassportElementAddress,
}

#[deprecated]
pub type RTDPassportElementAddressBuilder = PassportElementAddressBuilder;

impl PassportElementAddressBuilder {
    pub fn build(&self) -> PassportElementAddress {
        self.inner.clone()
    }

    pub fn address<T: AsRef<Address>>(&mut self, address: T) -> &mut Self {
        self.inner.address = address.as_ref().clone();
        self
    }
}

impl AsRef<PassportElementAddress> for PassportElementAddress {
    fn as_ref(&self) -> &PassportElementAddress {
        self
    }
}

impl AsRef<PassportElementAddress> for PassportElementAddressBuilder {
    fn as_ref(&self) -> &PassportElementAddress {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's bank statement
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementBankStatement {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Bank statement
    bank_statement: PersonalDocument,
}

impl RObject for PassportElementBankStatement {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElement for PassportElementBankStatement {}

impl PassportElementBankStatement {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportElementBankStatementBuilder {
        let mut inner = PassportElementBankStatement::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportElementBankStatementBuilder { inner }
    }

    pub fn bank_statement(&self) -> &PersonalDocument {
        &self.bank_statement
    }
}

#[doc(hidden)]
pub struct PassportElementBankStatementBuilder {
    inner: PassportElementBankStatement,
}

#[deprecated]
pub type RTDPassportElementBankStatementBuilder = PassportElementBankStatementBuilder;

impl PassportElementBankStatementBuilder {
    pub fn build(&self) -> PassportElementBankStatement {
        self.inner.clone()
    }

    pub fn bank_statement<T: AsRef<PersonalDocument>>(&mut self, bank_statement: T) -> &mut Self {
        self.inner.bank_statement = bank_statement.as_ref().clone();
        self
    }
}

impl AsRef<PassportElementBankStatement> for PassportElementBankStatement {
    fn as_ref(&self) -> &PassportElementBankStatement {
        self
    }
}

impl AsRef<PassportElementBankStatement> for PassportElementBankStatementBuilder {
    fn as_ref(&self) -> &PassportElementBankStatement {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's driver license
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementDriverLicense {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Driver license
    driver_license: IdentityDocument,
}

impl RObject for PassportElementDriverLicense {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElement for PassportElementDriverLicense {}

impl PassportElementDriverLicense {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportElementDriverLicenseBuilder {
        let mut inner = PassportElementDriverLicense::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportElementDriverLicenseBuilder { inner }
    }

    pub fn driver_license(&self) -> &IdentityDocument {
        &self.driver_license
    }
}

#[doc(hidden)]
pub struct PassportElementDriverLicenseBuilder {
    inner: PassportElementDriverLicense,
}

#[deprecated]
pub type RTDPassportElementDriverLicenseBuilder = PassportElementDriverLicenseBuilder;

impl PassportElementDriverLicenseBuilder {
    pub fn build(&self) -> PassportElementDriverLicense {
        self.inner.clone()
    }

    pub fn driver_license<T: AsRef<IdentityDocument>>(&mut self, driver_license: T) -> &mut Self {
        self.inner.driver_license = driver_license.as_ref().clone();
        self
    }
}

impl AsRef<PassportElementDriverLicense> for PassportElementDriverLicense {
    fn as_ref(&self) -> &PassportElementDriverLicense {
        self
    }
}

impl AsRef<PassportElementDriverLicense> for PassportElementDriverLicenseBuilder {
    fn as_ref(&self) -> &PassportElementDriverLicense {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's email address
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementEmailAddress {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Email address

    #[serde(default)]
    email_address: String,
}

impl RObject for PassportElementEmailAddress {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElement for PassportElementEmailAddress {}

impl PassportElementEmailAddress {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportElementEmailAddressBuilder {
        let mut inner = PassportElementEmailAddress::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportElementEmailAddressBuilder { inner }
    }

    pub fn email_address(&self) -> &String {
        &self.email_address
    }
}

#[doc(hidden)]
pub struct PassportElementEmailAddressBuilder {
    inner: PassportElementEmailAddress,
}

#[deprecated]
pub type RTDPassportElementEmailAddressBuilder = PassportElementEmailAddressBuilder;

impl PassportElementEmailAddressBuilder {
    pub fn build(&self) -> PassportElementEmailAddress {
        self.inner.clone()
    }

    pub fn email_address<T: AsRef<str>>(&mut self, email_address: T) -> &mut Self {
        self.inner.email_address = email_address.as_ref().to_string();
        self
    }
}

impl AsRef<PassportElementEmailAddress> for PassportElementEmailAddress {
    fn as_ref(&self) -> &PassportElementEmailAddress {
        self
    }
}

impl AsRef<PassportElementEmailAddress> for PassportElementEmailAddressBuilder {
    fn as_ref(&self) -> &PassportElementEmailAddress {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's identity card
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementIdentityCard {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Identity card
    identity_card: IdentityDocument,
}

impl RObject for PassportElementIdentityCard {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElement for PassportElementIdentityCard {}

impl PassportElementIdentityCard {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportElementIdentityCardBuilder {
        let mut inner = PassportElementIdentityCard::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportElementIdentityCardBuilder { inner }
    }

    pub fn identity_card(&self) -> &IdentityDocument {
        &self.identity_card
    }
}

#[doc(hidden)]
pub struct PassportElementIdentityCardBuilder {
    inner: PassportElementIdentityCard,
}

#[deprecated]
pub type RTDPassportElementIdentityCardBuilder = PassportElementIdentityCardBuilder;

impl PassportElementIdentityCardBuilder {
    pub fn build(&self) -> PassportElementIdentityCard {
        self.inner.clone()
    }

    pub fn identity_card<T: AsRef<IdentityDocument>>(&mut self, identity_card: T) -> &mut Self {
        self.inner.identity_card = identity_card.as_ref().clone();
        self
    }
}

impl AsRef<PassportElementIdentityCard> for PassportElementIdentityCard {
    fn as_ref(&self) -> &PassportElementIdentityCard {
        self
    }
}

impl AsRef<PassportElementIdentityCard> for PassportElementIdentityCardBuilder {
    fn as_ref(&self) -> &PassportElementIdentityCard {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's internal passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementInternalPassport {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Internal passport
    internal_passport: IdentityDocument,
}

impl RObject for PassportElementInternalPassport {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElement for PassportElementInternalPassport {}

impl PassportElementInternalPassport {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportElementInternalPassportBuilder {
        let mut inner = PassportElementInternalPassport::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportElementInternalPassportBuilder { inner }
    }

    pub fn internal_passport(&self) -> &IdentityDocument {
        &self.internal_passport
    }
}

#[doc(hidden)]
pub struct PassportElementInternalPassportBuilder {
    inner: PassportElementInternalPassport,
}

#[deprecated]
pub type RTDPassportElementInternalPassportBuilder = PassportElementInternalPassportBuilder;

impl PassportElementInternalPassportBuilder {
    pub fn build(&self) -> PassportElementInternalPassport {
        self.inner.clone()
    }

    pub fn internal_passport<T: AsRef<IdentityDocument>>(
        &mut self,
        internal_passport: T,
    ) -> &mut Self {
        self.inner.internal_passport = internal_passport.as_ref().clone();
        self
    }
}

impl AsRef<PassportElementInternalPassport> for PassportElementInternalPassport {
    fn as_ref(&self) -> &PassportElementInternalPassport {
        self
    }
}

impl AsRef<PassportElementInternalPassport> for PassportElementInternalPassportBuilder {
    fn as_ref(&self) -> &PassportElementInternalPassport {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's passport
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementPassport {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Passport
    passport: IdentityDocument,
}

impl RObject for PassportElementPassport {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElement for PassportElementPassport {}

impl PassportElementPassport {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportElementPassportBuilder {
        let mut inner = PassportElementPassport::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportElementPassportBuilder { inner }
    }

    pub fn passport(&self) -> &IdentityDocument {
        &self.passport
    }
}

#[doc(hidden)]
pub struct PassportElementPassportBuilder {
    inner: PassportElementPassport,
}

#[deprecated]
pub type RTDPassportElementPassportBuilder = PassportElementPassportBuilder;

impl PassportElementPassportBuilder {
    pub fn build(&self) -> PassportElementPassport {
        self.inner.clone()
    }

    pub fn passport<T: AsRef<IdentityDocument>>(&mut self, passport: T) -> &mut Self {
        self.inner.passport = passport.as_ref().clone();
        self
    }
}

impl AsRef<PassportElementPassport> for PassportElementPassport {
    fn as_ref(&self) -> &PassportElementPassport {
        self
    }
}

impl AsRef<PassportElementPassport> for PassportElementPassportBuilder {
    fn as_ref(&self) -> &PassportElementPassport {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's passport registration pages
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementPassportRegistration {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Passport registration pages
    passport_registration: PersonalDocument,
}

impl RObject for PassportElementPassportRegistration {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElement for PassportElementPassportRegistration {}

impl PassportElementPassportRegistration {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportElementPassportRegistrationBuilder {
        let mut inner = PassportElementPassportRegistration::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportElementPassportRegistrationBuilder { inner }
    }

    pub fn passport_registration(&self) -> &PersonalDocument {
        &self.passport_registration
    }
}

#[doc(hidden)]
pub struct PassportElementPassportRegistrationBuilder {
    inner: PassportElementPassportRegistration,
}

#[deprecated]
pub type RTDPassportElementPassportRegistrationBuilder = PassportElementPassportRegistrationBuilder;

impl PassportElementPassportRegistrationBuilder {
    pub fn build(&self) -> PassportElementPassportRegistration {
        self.inner.clone()
    }

    pub fn passport_registration<T: AsRef<PersonalDocument>>(
        &mut self,
        passport_registration: T,
    ) -> &mut Self {
        self.inner.passport_registration = passport_registration.as_ref().clone();
        self
    }
}

impl AsRef<PassportElementPassportRegistration> for PassportElementPassportRegistration {
    fn as_ref(&self) -> &PassportElementPassportRegistration {
        self
    }
}

impl AsRef<PassportElementPassportRegistration> for PassportElementPassportRegistrationBuilder {
    fn as_ref(&self) -> &PassportElementPassportRegistration {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's personal details
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementPersonalDetails {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Personal details of the user
    personal_details: PersonalDetails,
}

impl RObject for PassportElementPersonalDetails {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElement for PassportElementPersonalDetails {}

impl PassportElementPersonalDetails {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportElementPersonalDetailsBuilder {
        let mut inner = PassportElementPersonalDetails::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportElementPersonalDetailsBuilder { inner }
    }

    pub fn personal_details(&self) -> &PersonalDetails {
        &self.personal_details
    }
}

#[doc(hidden)]
pub struct PassportElementPersonalDetailsBuilder {
    inner: PassportElementPersonalDetails,
}

#[deprecated]
pub type RTDPassportElementPersonalDetailsBuilder = PassportElementPersonalDetailsBuilder;

impl PassportElementPersonalDetailsBuilder {
    pub fn build(&self) -> PassportElementPersonalDetails {
        self.inner.clone()
    }

    pub fn personal_details<T: AsRef<PersonalDetails>>(
        &mut self,
        personal_details: T,
    ) -> &mut Self {
        self.inner.personal_details = personal_details.as_ref().clone();
        self
    }
}

impl AsRef<PassportElementPersonalDetails> for PassportElementPersonalDetails {
    fn as_ref(&self) -> &PassportElementPersonalDetails {
        self
    }
}

impl AsRef<PassportElementPersonalDetails> for PassportElementPersonalDetailsBuilder {
    fn as_ref(&self) -> &PassportElementPersonalDetails {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's phone number
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementPhoneNumber {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Phone number

    #[serde(default)]
    phone_number: String,
}

impl RObject for PassportElementPhoneNumber {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElement for PassportElementPhoneNumber {}

impl PassportElementPhoneNumber {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportElementPhoneNumberBuilder {
        let mut inner = PassportElementPhoneNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportElementPhoneNumberBuilder { inner }
    }

    pub fn phone_number(&self) -> &String {
        &self.phone_number
    }
}

#[doc(hidden)]
pub struct PassportElementPhoneNumberBuilder {
    inner: PassportElementPhoneNumber,
}

#[deprecated]
pub type RTDPassportElementPhoneNumberBuilder = PassportElementPhoneNumberBuilder;

impl PassportElementPhoneNumberBuilder {
    pub fn build(&self) -> PassportElementPhoneNumber {
        self.inner.clone()
    }

    pub fn phone_number<T: AsRef<str>>(&mut self, phone_number: T) -> &mut Self {
        self.inner.phone_number = phone_number.as_ref().to_string();
        self
    }
}

impl AsRef<PassportElementPhoneNumber> for PassportElementPhoneNumber {
    fn as_ref(&self) -> &PassportElementPhoneNumber {
        self
    }
}

impl AsRef<PassportElementPhoneNumber> for PassportElementPhoneNumberBuilder {
    fn as_ref(&self) -> &PassportElementPhoneNumber {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's rental agreement
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementRentalAgreement {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Rental agreement
    rental_agreement: PersonalDocument,
}

impl RObject for PassportElementRentalAgreement {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElement for PassportElementRentalAgreement {}

impl PassportElementRentalAgreement {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportElementRentalAgreementBuilder {
        let mut inner = PassportElementRentalAgreement::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportElementRentalAgreementBuilder { inner }
    }

    pub fn rental_agreement(&self) -> &PersonalDocument {
        &self.rental_agreement
    }
}

#[doc(hidden)]
pub struct PassportElementRentalAgreementBuilder {
    inner: PassportElementRentalAgreement,
}

#[deprecated]
pub type RTDPassportElementRentalAgreementBuilder = PassportElementRentalAgreementBuilder;

impl PassportElementRentalAgreementBuilder {
    pub fn build(&self) -> PassportElementRentalAgreement {
        self.inner.clone()
    }

    pub fn rental_agreement<T: AsRef<PersonalDocument>>(
        &mut self,
        rental_agreement: T,
    ) -> &mut Self {
        self.inner.rental_agreement = rental_agreement.as_ref().clone();
        self
    }
}

impl AsRef<PassportElementRentalAgreement> for PassportElementRentalAgreement {
    fn as_ref(&self) -> &PassportElementRentalAgreement {
        self
    }
}

impl AsRef<PassportElementRentalAgreement> for PassportElementRentalAgreementBuilder {
    fn as_ref(&self) -> &PassportElementRentalAgreement {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's temporary registration
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementTemporaryRegistration {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Temporary registration
    temporary_registration: PersonalDocument,
}

impl RObject for PassportElementTemporaryRegistration {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElement for PassportElementTemporaryRegistration {}

impl PassportElementTemporaryRegistration {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportElementTemporaryRegistrationBuilder {
        let mut inner = PassportElementTemporaryRegistration::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportElementTemporaryRegistrationBuilder { inner }
    }

    pub fn temporary_registration(&self) -> &PersonalDocument {
        &self.temporary_registration
    }
}

#[doc(hidden)]
pub struct PassportElementTemporaryRegistrationBuilder {
    inner: PassportElementTemporaryRegistration,
}

#[deprecated]
pub type RTDPassportElementTemporaryRegistrationBuilder =
    PassportElementTemporaryRegistrationBuilder;

impl PassportElementTemporaryRegistrationBuilder {
    pub fn build(&self) -> PassportElementTemporaryRegistration {
        self.inner.clone()
    }

    pub fn temporary_registration<T: AsRef<PersonalDocument>>(
        &mut self,
        temporary_registration: T,
    ) -> &mut Self {
        self.inner.temporary_registration = temporary_registration.as_ref().clone();
        self
    }
}

impl AsRef<PassportElementTemporaryRegistration> for PassportElementTemporaryRegistration {
    fn as_ref(&self) -> &PassportElementTemporaryRegistration {
        self
    }
}

impl AsRef<PassportElementTemporaryRegistration> for PassportElementTemporaryRegistrationBuilder {
    fn as_ref(&self) -> &PassportElementTemporaryRegistration {
        &self.inner
    }
}

/// A Telegram Passport element containing the user's utility bill
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PassportElementUtilityBill {
    #[doc(hidden)]
    #[serde(rename(serialize = "@extra", deserialize = "@extra"))]
    extra: Option<String>,
    #[serde(rename(serialize = "@client_id", deserialize = "@client_id"))]
    client_id: Option<i32>,
    /// Utility bill
    utility_bill: PersonalDocument,
}

impl RObject for PassportElementUtilityBill {
    #[doc(hidden)]
    fn extra(&self) -> Option<&str> {
        self.extra.as_deref()
    }
    #[doc(hidden)]
    fn client_id(&self) -> Option<i32> {
        self.client_id
    }
}

impl TDPassportElement for PassportElementUtilityBill {}

impl PassportElementUtilityBill {
    pub fn from_json<S: AsRef<str>>(json: S) -> Result<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> PassportElementUtilityBillBuilder {
        let mut inner = PassportElementUtilityBill::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        PassportElementUtilityBillBuilder { inner }
    }

    pub fn utility_bill(&self) -> &PersonalDocument {
        &self.utility_bill
    }
}

#[doc(hidden)]
pub struct PassportElementUtilityBillBuilder {
    inner: PassportElementUtilityBill,
}

#[deprecated]
pub type RTDPassportElementUtilityBillBuilder = PassportElementUtilityBillBuilder;

impl PassportElementUtilityBillBuilder {
    pub fn build(&self) -> PassportElementUtilityBill {
        self.inner.clone()
    }

    pub fn utility_bill<T: AsRef<PersonalDocument>>(&mut self, utility_bill: T) -> &mut Self {
        self.inner.utility_bill = utility_bill.as_ref().clone();
        self
    }
}

impl AsRef<PassportElementUtilityBill> for PassportElementUtilityBill {
    fn as_ref(&self) -> &PassportElementUtilityBill {
        self
    }
}

impl AsRef<PassportElementUtilityBill> for PassportElementUtilityBillBuilder {
    fn as_ref(&self) -> &PassportElementUtilityBill {
        &self.inner
    }
}
