// Module responsavel por fazer a requisição

use crate::actors::SyncActor;
use actix::{Handler, Message};
use serde_derive::Deserialize;

pub struct CEP(pub String);

impl Message for CEP {
    type Result = Result<String, ()>;
}
impl Handler<CEP> for SyncActor {
    type Result = Result<String, ()>;
    fn handle(&mut self, msg: CEP, _: &mut Self::Context) -> Self::Result {
        use reqwest::blocking::Client;
        let client = Client::new();
        let uri = format!("https://viacep.com.br/ws/{}/json", msg.0);
        let resp = client.get(&uri).send().unwrap();
        let city: Local = resp.json().unwrap();
        Ok(city.localidade)
    }
}

#[derive(Deserialize)]
struct Local {
    localidade: String,
}
