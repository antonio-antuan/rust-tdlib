use crate::errors::RTDResult;
use crate::tdjson;
use crate::types::RFunction;

pub trait TdLibClient {
    fn send<Fnc: RFunction>(&self, client_id: tdjson::ClientId, fnc: Fnc) -> RTDResult<()>;
    fn receive(&self, timeout: f64) -> Option<String>;
    fn execute<Fnc: RFunction>(&self, fnc: Fnc) -> RTDResult<Option<String>>;
    fn new_client(&self) -> tdjson::ClientId;
}

#[derive(Clone, Debug)]
pub struct RawApi;

impl Default for RawApi {
    fn default() -> Self {
        Self
    }
}

impl TdLibClient for RawApi {
    fn send<Fnc: RFunction>(&self, client_id: tdjson::ClientId, fnc: Fnc) -> RTDResult<()> {
        let json = fnc.to_json()?;
        tdjson::send(client_id, &json[..]);
        Ok(())
    }

    fn receive(&self, timeout: f64) -> Option<String> {
        tdjson::receive(timeout)
    }

    fn execute<Fnc: RFunction>(&self, fnc: Fnc) -> RTDResult<Option<String>> {
        let json = fnc.to_json()?;
        Ok(tdjson::execute(&json[..]))
    }

    fn new_client(&self) -> tdjson::ClientId {
        tdjson::new_client()
    }
}

impl RawApi {
    pub fn new() -> Self {
        Self {}
    }
}
