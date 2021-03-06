use actix::{Handler, Message};
use transistor::{client::Crux, types::CruxId};

use crate::actors::AsyncActor;

use super::register::Register;

#[derive(Clone)]
pub struct Entity(pub String);

impl Message for Entity {
    type Result = Result<Register, ()>;
}
impl Handler<Entity> for AsyncActor {
    type Result = Result<Register, ()>;
    fn handle(&mut self, msg: Entity, _: &mut Self::Context) -> Self::Result {
        let client = Crux::new("127.0.0.1", "3000").http_client();
        let id = CruxId::new(&msg.0);
        let entity = client.entity(edn_rs::to_string(id)).unwrap();
        Ok(edn_rs::from_edn::<Register>(&entity).unwrap())
    }
}
