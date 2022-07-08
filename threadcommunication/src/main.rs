// Comunicção via thread pode ser feita via channels
// Os dados são processados via FIFO(o primeiro a entrar é o primeiro a sair)
// Nesse mecanismo, uma copia dos dados é feita para ser compartilhada com a thread recebedora, e por isso não se recomenda usar dados extensos
// Para	 podermos	 enviar	 dados	 por	 channels,	 é	 preciso	 garantir
// que	 o	 que	 estamos	 enviando	implemente	 a	 trait	 	send	,	 pois	 ela
// garante	 a	 transferência	 de	 ownership	 segura	 entre	 as	 threads.
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

#[derive(Debug)]
struct MyPi {
    pi: f32,
}
// Nesse caso não precisamos implementar a trait Send pois todos os campos de MyPi ja a implementao
impl MyPi {
    fn new() -> Self {
        Self { pi: 3.1415f32 }
    }
}
fn main() {
    let (tx, rx): (Sender<MyPi>, Receiver<MyPi>) = channel();
    thread::spawn(move || {
        tx.send(MyPi::new()).unwrap();
    });
    let pi = rx.recv().unwrap();
    println!("{:?}", pi.pi)
}

fn criar_canal(dado: i32) -> Receiver<i32> {
    let (tx, rx) = channel();
    tx.send(dado).ok().expect("Dado não pode ser enviado");
    rx
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn verificar_func_criar_canal() {
        let expected = Some(8000);
        let rx = criar_canal(8000);
        assert_eq!(expected, rx.recv().ok());
    }
}
