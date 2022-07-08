// Vamos criar um contado que aumenta
// cada vez que recebe uma mensagem Ping

// Vamos criar um ator para gerenciar este Pings
// Devemos implementar a trait Actor, pois um ator é basicament
// uma struct que implementa esse trait:

use actix::prelude::*;

struct NossoActor {
    count: usize,
}

impl Actor for NossoActor {
    type Context = Context<Self>;
}
// Agora vamos implementar a mensage Ping.
// Uma mensagem é qualquer struct que implementa o tipo Message
// e seu principal objetivo é definir o tipo de retorno
struct Ping(usize);
impl Message for Ping {
    type Result = usize;
}

// Agora precisamos declarar que NossoActor aceita o tipo de mensagem Ping
//e sabe lidar com ela atraves da trait Handler:
impl Handler<Ping> for NossoActor {
    type Result = usize;
    fn handle(&mut self, msg: Ping, _ctx: &mut Self::Context) -> Self::Result {
        self.count += msg.0;
        self.count
    }
}

// Agora precisamos somente inicar um ator e enviar um Ping para ele.
#[actix_rt::main]
async fn main() {
    let addr = NossoActor { count: 10 }.start();
    let res = addr.send(Ping(10)).await;
    println!("Resultado: {}", res.unwrap() == 20);
    System::current().stop();
}
