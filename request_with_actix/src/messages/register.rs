// Modulo responsavel por fazer a conexao com o banco de dados
use crate::actors::SyncActor;
use actix::{Handler, Message};
use edn_rs::{serialize::Serialize, Deserialize, Edn, EdnError};
use transistor::{
    client::Crux,
    types::{Action, CruxId},
};

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Register {
    crux__db__id: CruxId,
    name: String,
    alias: String,
    postal_code: String,
    city: String,
}
impl Serialize for Register {
    fn serialize(self) -> String {
        format!("{:?}", self)
    }
}
impl Deserialize for Register {
    fn deserialize(edn: &Edn) -> Result<Self, EdnError> {
        Ok(Self {
            crux__db__id: edn_rs::from_edn(&edn[":cruxId"])?,
            name: edn_rs::from_edn(&edn[":name"])?,
            alias: edn_rs::from_edn(&edn[":alias"])?,
            postal_code: edn_rs::from_edn(&edn[":postal_code"])?,
            city: edn_rs::from_edn(&edn[":city"])?,
        })
    }
}
impl Register {
    pub fn new(name: &str, alias: &str, cep: &str, city: &str) -> Self {
        Self {
            crux__db__id: CruxId::new(alias),
            name: name.to_string(),
            alias: alias.to_string(),
            postal_code: cep.to_string(),
            city: city.to_string(),
        }
    }
}
// Fazendo nosso register ser uma mensagem
impl Message for Register {
    type Result = Result<bool, ()>;
}

impl Handler<Register> for SyncActor {
    type Result = Result<bool, ()>;
    fn handle(&mut self, msg: Register, _: &mut Self::Context) -> Self::Result {
        // Obtendo um http client do crux
        let client = Crux::new("127.0.0.1", "3000").http_client();

        let action = Action::Put(edn_rs::to_string(msg), None);
        // Escrevendo um register no Crux
        let tx_log = client.tx_log(vec![action]);
        match tx_log {
            Ok(tx) => {
                println!("{:?}", tx);
                Ok(true)
            }
            _ => Err(()),
        }
    }
}
