use crate::errors::*;
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
    #[serde(rename(serialize = "getPassportElement", deserialize = "getPassportElement"))]
    GetPassportElement(GetPassportElement),
    /// A Telegram Passport element containing the user's address
    #[serde(rename(
        serialize = "passportElementAddress",
        deserialize = "passportElementAddress"
    ))]
    Address(PassportElementAddress),
    /// A Telegram Passport element containing the user's bank statement
    #[serde(rename(
        serialize = "passportElementBankStatement",
        deserialize = "passportElementBankStatement"
    ))]
    BankStatement(PassportElementBankStatement),
    /// A Telegram Passport element containing the user's driver license
    #[serde(rename(
        serialize = "passportElementDriverLicense",
        deserialize = "passportElementDriverLicense"
    ))]
    DriverLicense(PassportElementDriverLicense),
    /// A Telegram Passport element containing the user's email address
    #[serde(rename(
        serialize = "passportElementEmailAddress",
        deserialize = "passportElementEmailAddress"
    ))]
    EmailAddress(PassportElementEmailAddress),
    /// A Telegram Passport element containing the user's identity card
    #[serde(rename(
        serialize = "passportElementIdentityCard",
        deserialize = "passportElementIdentityCard"
    ))]
    IdentityCard(PassportElementIdentityCard),
    /// A Telegram Passport element containing the user's internal passport
    #[serde(rename(
        serialize = "passportElementInternalPassport",
        deserialize = "passportElementInternalPassport"
    ))]
    InternalPassport(PassportElementInternalPassport),
    /// A Telegram Passport element containing the user's passport
    #[serde(rename(
        serialize = "passportElementPassport",
        deserialize = "passportElementPassport"
    ))]
    Passport(PassportElementPassport),
    /// A Telegram Passport element containing the user's passport registration pages
    #[serde(rename(
        serialize = "passportElementPassportRegistration",
        deserialize = "passportElementPassportRegistration"
    ))]
    PassportRegistration(PassportElementPassportRegistration),
    /// A Telegram Passport element containing the user's personal details
    #[serde(rename(
        serialize = "passportElementPersonalDetails",
        deserialize = "passportElementPersonalDetails"
    ))]
    PersonalDetails(PassportElementPersonalDetails),
    /// A Telegram Passport element containing the user's phone number
    #[serde(rename(
        serialize = "passportElementPhoneNumber",
        deserialize = "passportElementPhoneNumber"
    ))]
    PhoneNumber(PassportElementPhoneNumber),
    /// A Telegram Passport element containing the user's rental agreement
    #[serde(rename(
        serialize = "passportElementRentalAgreement",
        deserialize = "passportElementRentalAgreement"
    ))]
    RentalAgreement(PassportElementRentalAgreement),
    /// A Telegram Passport element containing the user's temporary registration
    #[serde(rename(
        serialize = "passportElementTemporaryRegistration",
        deserialize = "passportElementTemporaryRegistration"
    ))]
    TemporaryRegistration(PassportElementTemporaryRegistration),
    /// A Telegram Passport element containing the user's utility bill
    #[serde(rename(
        serialize = "passportElementUtilityBill",
        deserialize = "passportElementUtilityBill"
    ))]
    UtilityBill(PassportElementUtilityBill),
    /// Adds an element to the user's Telegram Passport. May return an error with a message "PHONE_VERIFICATION_NEEDED" or "EMAIL_VERIFICATION_NEEDED" if the chosen phone number or the chosen email address must be verified first
    #[serde(rename(serialize = "setPassportElement", deserialize = "setPassportElement"))]
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementAddressBuilder {
        let mut inner = PassportElementAddress::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportElementAddressBuilder { inner }
    }

    pub fn address(&self) -> &Address {
        &self.address
    }
}

#[doc(hidden)]
pub struct RTDPassportElementAddressBuilder {
    inner: PassportElementAddress,
}

impl RTDPassportElementAddressBuilder {
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

impl AsRef<PassportElementAddress> for RTDPassportElementAddressBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementBankStatementBuilder {
        let mut inner = PassportElementBankStatement::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportElementBankStatementBuilder { inner }
    }

    pub fn bank_statement(&self) -> &PersonalDocument {
        &self.bank_statement
    }
}

#[doc(hidden)]
pub struct RTDPassportElementBankStatementBuilder {
    inner: PassportElementBankStatement,
}

impl RTDPassportElementBankStatementBuilder {
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

impl AsRef<PassportElementBankStatement> for RTDPassportElementBankStatementBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementDriverLicenseBuilder {
        let mut inner = PassportElementDriverLicense::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportElementDriverLicenseBuilder { inner }
    }

    pub fn driver_license(&self) -> &IdentityDocument {
        &self.driver_license
    }
}

#[doc(hidden)]
pub struct RTDPassportElementDriverLicenseBuilder {
    inner: PassportElementDriverLicense,
}

impl RTDPassportElementDriverLicenseBuilder {
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

impl AsRef<PassportElementDriverLicense> for RTDPassportElementDriverLicenseBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementEmailAddressBuilder {
        let mut inner = PassportElementEmailAddress::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportElementEmailAddressBuilder { inner }
    }

    pub fn email_address(&self) -> &String {
        &self.email_address
    }
}

#[doc(hidden)]
pub struct RTDPassportElementEmailAddressBuilder {
    inner: PassportElementEmailAddress,
}

impl RTDPassportElementEmailAddressBuilder {
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

impl AsRef<PassportElementEmailAddress> for RTDPassportElementEmailAddressBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementIdentityCardBuilder {
        let mut inner = PassportElementIdentityCard::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportElementIdentityCardBuilder { inner }
    }

    pub fn identity_card(&self) -> &IdentityDocument {
        &self.identity_card
    }
}

#[doc(hidden)]
pub struct RTDPassportElementIdentityCardBuilder {
    inner: PassportElementIdentityCard,
}

impl RTDPassportElementIdentityCardBuilder {
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

impl AsRef<PassportElementIdentityCard> for RTDPassportElementIdentityCardBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementInternalPassportBuilder {
        let mut inner = PassportElementInternalPassport::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportElementInternalPassportBuilder { inner }
    }

    pub fn internal_passport(&self) -> &IdentityDocument {
        &self.internal_passport
    }
}

#[doc(hidden)]
pub struct RTDPassportElementInternalPassportBuilder {
    inner: PassportElementInternalPassport,
}

impl RTDPassportElementInternalPassportBuilder {
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

impl AsRef<PassportElementInternalPassport> for RTDPassportElementInternalPassportBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementPassportBuilder {
        let mut inner = PassportElementPassport::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportElementPassportBuilder { inner }
    }

    pub fn passport(&self) -> &IdentityDocument {
        &self.passport
    }
}

#[doc(hidden)]
pub struct RTDPassportElementPassportBuilder {
    inner: PassportElementPassport,
}

impl RTDPassportElementPassportBuilder {
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

impl AsRef<PassportElementPassport> for RTDPassportElementPassportBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementPassportRegistrationBuilder {
        let mut inner = PassportElementPassportRegistration::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportElementPassportRegistrationBuilder { inner }
    }

    pub fn passport_registration(&self) -> &PersonalDocument {
        &self.passport_registration
    }
}

#[doc(hidden)]
pub struct RTDPassportElementPassportRegistrationBuilder {
    inner: PassportElementPassportRegistration,
}

impl RTDPassportElementPassportRegistrationBuilder {
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

impl AsRef<PassportElementPassportRegistration> for RTDPassportElementPassportRegistrationBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementPersonalDetailsBuilder {
        let mut inner = PassportElementPersonalDetails::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportElementPersonalDetailsBuilder { inner }
    }

    pub fn personal_details(&self) -> &PersonalDetails {
        &self.personal_details
    }
}

#[doc(hidden)]
pub struct RTDPassportElementPersonalDetailsBuilder {
    inner: PassportElementPersonalDetails,
}

impl RTDPassportElementPersonalDetailsBuilder {
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

impl AsRef<PassportElementPersonalDetails> for RTDPassportElementPersonalDetailsBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementPhoneNumberBuilder {
        let mut inner = PassportElementPhoneNumber::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportElementPhoneNumberBuilder { inner }
    }

    pub fn phone_number(&self) -> &String {
        &self.phone_number
    }
}

#[doc(hidden)]
pub struct RTDPassportElementPhoneNumberBuilder {
    inner: PassportElementPhoneNumber,
}

impl RTDPassportElementPhoneNumberBuilder {
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

impl AsRef<PassportElementPhoneNumber> for RTDPassportElementPhoneNumberBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementRentalAgreementBuilder {
        let mut inner = PassportElementRentalAgreement::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportElementRentalAgreementBuilder { inner }
    }

    pub fn rental_agreement(&self) -> &PersonalDocument {
        &self.rental_agreement
    }
}

#[doc(hidden)]
pub struct RTDPassportElementRentalAgreementBuilder {
    inner: PassportElementRentalAgreement,
}

impl RTDPassportElementRentalAgreementBuilder {
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

impl AsRef<PassportElementRentalAgreement> for RTDPassportElementRentalAgreementBuilder {
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementTemporaryRegistrationBuilder {
        let mut inner = PassportElementTemporaryRegistration::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportElementTemporaryRegistrationBuilder { inner }
    }

    pub fn temporary_registration(&self) -> &PersonalDocument {
        &self.temporary_registration
    }
}

#[doc(hidden)]
pub struct RTDPassportElementTemporaryRegistrationBuilder {
    inner: PassportElementTemporaryRegistration,
}

impl RTDPassportElementTemporaryRegistrationBuilder {
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

impl AsRef<PassportElementTemporaryRegistration>
    for RTDPassportElementTemporaryRegistrationBuilder
{
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
    pub fn from_json<S: AsRef<str>>(json: S) -> RTDResult<Self> {
        Ok(serde_json::from_str(json.as_ref())?)
    }
    pub fn builder() -> RTDPassportElementUtilityBillBuilder {
        let mut inner = PassportElementUtilityBill::default();
        inner.extra = Some(Uuid::new_v4().to_string());

        RTDPassportElementUtilityBillBuilder { inner }
    }

    pub fn utility_bill(&self) -> &PersonalDocument {
        &self.utility_bill
    }
}

#[doc(hidden)]
pub struct RTDPassportElementUtilityBillBuilder {
    inner: PassportElementUtilityBill,
}

impl RTDPassportElementUtilityBillBuilder {
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

impl AsRef<PassportElementUtilityBill> for RTDPassportElementUtilityBillBuilder {
    fn as_ref(&self) -> &PassportElementUtilityBill {
        &self.inner
    }
}
