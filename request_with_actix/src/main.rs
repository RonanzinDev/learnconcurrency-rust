use actix::prelude::*;
mod actors;
use actors::{AsyncActor, SyncActor};
mod messages;
use messages::{cep::CEP, entity::Entity, register::Register};

// Vamos criar um ator que consigo fazer um request para uma API contento o CEP de alguem
//e nos retorna a cidade daquele CEP.
#[actix_rt::main]
async fn main() {
    let cpf = "05409000";
    let alias = "11999887766";
    let addr_sync = SyncArbiter::start(3, || SyncActor);
    let addr_async = AsyncActor {}.start();
    let city = addr_sync.send(CEP(cpf.to_string())).await.unwrap().unwrap();

    let register = addr_sync
        .send(Register::new("julia", alias, cpf, &city))
        .await
        .unwrap();

    let entity = addr_async.send(Entity(alias.to_string())).await.unwrap();
    println!("{:?}", entity);
    System::current().stop();
}
