use crate::types::RObject;

const PLZ_POST_ISSUES: &'static str =
    "PLEASE POST AN ISSUE TO https://github.com/fewensa/telegram-client/issues";

const TELEGRAM_DATA_FAIL: &'static str = "TELEGRAM DATA FAIL. IF YOU SEE THIS MESSAGE,";

pub fn no_data_returned_from_tdlib() -> &'static str {
    "No data returned from tdlib"
}

pub fn please_post_issues() -> &'static str {
    PLZ_POST_ISSUES
}

pub fn data_fail_with_json<S: AsRef<str>>(json: S) -> String {
    format!(
        "{} {} \n INCLUDE THIS JSON => {}",
        TELEGRAM_DATA_FAIL,
        PLZ_POST_ISSUES,
        json.as_ref()
    )
}

pub fn data_fail_with_rtd<ROBJ: RObject>(robj: ROBJ) -> String {
    data_fail_with_json(robj.to_json().unwrap_or("".to_string()))
}
