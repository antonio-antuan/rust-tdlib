use crate::types::{RObject, TdType};
use futures::channel::oneshot;
use std::collections::HashMap;
use std::sync::RwLock;

lazy_static! {
    pub(super) static ref OBSERVER: Observer = Observer::new();
}

pub(super) struct Observer {
    channels: RwLock<HashMap<String, oneshot::Sender<TdType>>>,
}

impl Observer {
    fn new() -> Self {
        Self {
            channels: RwLock::new(HashMap::new()),
        }
    }

    pub fn notify(&self, payload: TdType) -> Option<TdType> {
        let extra = match &payload {
            TdType::AuthorizationState(value) => value.extra(),

            TdType::CanTransferOwnershipResult(value) => value.extra(),

            TdType::ChatStatistics(value) => value.extra(),

            TdType::CheckChatUsernameResult(value) => value.extra(),

            TdType::JsonValue(value) => value.extra(),

            TdType::LanguagePackStringValue(value) => value.extra(),

            TdType::LogStream(value) => value.extra(),

            TdType::LoginUrlInfo(value) => value.extra(),

            TdType::OptionValue(value) => value.extra(),

            TdType::PassportElement(value) => value.extra(),

            TdType::StatisticalGraph(value) => value.extra(),

            TdType::Update(value) => value.extra(),

            TdType::AccountTtl(value) => value.extra(),

            TdType::Animations(value) => value.extra(),

            TdType::AuthenticationCodeInfo(value) => value.extra(),

            TdType::AutoDownloadSettingsPresets(value) => value.extra(),

            TdType::Background(value) => value.extra(),

            TdType::Backgrounds(value) => value.extra(),

            TdType::BankCardInfo(value) => value.extra(),

            TdType::BasicGroup(value) => value.extra(),

            TdType::BasicGroupFullInfo(value) => value.extra(),

            TdType::CallId(value) => value.extra(),

            TdType::CallbackQueryAnswer(value) => value.extra(),

            TdType::Chat(value) => value.extra(),

            TdType::ChatAdministrators(value) => value.extra(),

            TdType::ChatEvents(value) => value.extra(),

            TdType::ChatFilter(value) => value.extra(),

            TdType::ChatFilterInfo(value) => value.extra(),

            TdType::ChatInviteLink(value) => value.extra(),

            TdType::ChatInviteLinkInfo(value) => value.extra(),

            TdType::ChatLists(value) => value.extra(),

            TdType::ChatMember(value) => value.extra(),

            TdType::ChatMembers(value) => value.extra(),

            TdType::ChatPhotos(value) => value.extra(),

            TdType::Chats(value) => value.extra(),

            TdType::ChatsNearby(value) => value.extra(),

            TdType::ConnectedWebsites(value) => value.extra(),

            TdType::Count(value) => value.extra(),

            TdType::Countries(value) => value.extra(),

            TdType::CustomRequestResult(value) => value.extra(),

            TdType::DatabaseStatistics(value) => value.extra(),

            TdType::DeepLinkInfo(value) => value.extra(),

            TdType::EmailAddressAuthenticationCodeInfo(value) => value.extra(),

            TdType::Emojis(value) => value.extra(),

            TdType::Error(value) => value.extra(),

            TdType::File(value) => value.extra(),

            TdType::FilePart(value) => value.extra(),

            TdType::FormattedText(value) => value.extra(),

            TdType::FoundMessages(value) => value.extra(),

            TdType::GameHighScores(value) => value.extra(),

            TdType::Hashtags(value) => value.extra(),

            TdType::HttpUrl(value) => value.extra(),

            TdType::ImportedContacts(value) => value.extra(),

            TdType::InlineQueryResults(value) => value.extra(),

            TdType::LanguagePackInfo(value) => value.extra(),

            TdType::LanguagePackStrings(value) => value.extra(),

            TdType::LocalizationTargetInfo(value) => value.extra(),

            TdType::LogTags(value) => value.extra(),

            TdType::LogVerbosityLevel(value) => value.extra(),

            TdType::Message(value) => value.extra(),

            TdType::MessageLink(value) => value.extra(),

            TdType::MessageLinkInfo(value) => value.extra(),

            TdType::MessageSenders(value) => value.extra(),

            TdType::MessageStatistics(value) => value.extra(),

            TdType::MessageThreadInfo(value) => value.extra(),

            TdType::Messages(value) => value.extra(),

            TdType::NetworkStatistics(value) => value.extra(),

            TdType::Ok(value) => value.extra(),

            TdType::OrderInfo(value) => value.extra(),

            TdType::PassportAuthorizationForm(value) => value.extra(),

            TdType::PassportElements(value) => value.extra(),

            TdType::PassportElementsWithErrors(value) => value.extra(),

            TdType::PasswordState(value) => value.extra(),

            TdType::PaymentForm(value) => value.extra(),

            TdType::PaymentReceipt(value) => value.extra(),

            TdType::PaymentResult(value) => value.extra(),

            TdType::PhoneNumberInfo(value) => value.extra(),

            TdType::Proxies(value) => value.extra(),

            TdType::Proxy(value) => value.extra(),

            TdType::PushReceiverId(value) => value.extra(),

            TdType::RecommendedChatFilters(value) => value.extra(),

            TdType::RecoveryEmailAddress(value) => value.extra(),

            TdType::ScopeNotificationSettings(value) => value.extra(),

            TdType::Seconds(value) => value.extra(),

            TdType::SecretChat(value) => value.extra(),

            TdType::Session(value) => value.extra(),

            TdType::Sessions(value) => value.extra(),

            TdType::StickerSet(value) => value.extra(),

            TdType::StickerSets(value) => value.extra(),

            TdType::Stickers(value) => value.extra(),

            TdType::StorageStatistics(value) => value.extra(),

            TdType::StorageStatisticsFast(value) => value.extra(),

            TdType::Supergroup(value) => value.extra(),

            TdType::SupergroupFullInfo(value) => value.extra(),

            TdType::TMeUrls(value) => value.extra(),

            TdType::TemporaryPasswordState(value) => value.extra(),

            TdType::TestBytes(value) => value.extra(),

            TdType::TestInt(value) => value.extra(),

            TdType::TestString(value) => value.extra(),

            TdType::TestVectorInt(value) => value.extra(),

            TdType::TestVectorIntObject(value) => value.extra(),

            TdType::TestVectorString(value) => value.extra(),

            TdType::TestVectorStringObject(value) => value.extra(),

            TdType::Text(value) => value.extra(),

            TdType::TextEntities(value) => value.extra(),

            TdType::Updates(value) => value.extra(),

            TdType::User(value) => value.extra(),

            TdType::UserFullInfo(value) => value.extra(),

            TdType::UserPrivacySettingRules(value) => value.extra(),

            TdType::Users(value) => value.extra(),

            TdType::ValidatedOrderInfo(value) => value.extra(),

            TdType::WebPage(value) => value.extra(),

            TdType::WebPageInstantView(value) => value.extra(),

            _ => None,
        };
        match extra {
            None => {
                trace!("no extra for payload {:?}", payload);
                Some(payload)
            }
            Some(extra) => {
                let mut map = self.channels.write().unwrap();
                match map.remove(&extra) {
                    None => {
                        trace!("no subscribers for {}", extra);
                        Some(payload)
                    }
                    Some(sender) => {
                        trace!("signal send for {}", extra);
                        sender.send(payload).unwrap();
                        None
                    }
                }
            }
        }
    }

    pub fn subscribe(&self, extra: &str) -> oneshot::Receiver<TdType> {
        let (sender, receiver) = oneshot::channel::<TdType>();
        match self.channels.write() {
            Ok(mut map) => {
                map.insert(extra.to_string(), sender);
                trace!("subscribed for {}", extra);
            }
            _ => {
                warn!("can't acquire lock for notifier map");
            }
        };
        receiver
    }

    pub fn unsubscribe(&self, extra: &str) {
        if let Ok(mut map) = self.channels.write() {
            trace!("remove {} subscription", &extra);
            map.remove(extra);
        };
    }
}
