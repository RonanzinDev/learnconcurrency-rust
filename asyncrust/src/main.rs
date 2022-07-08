// EXEMPLO DE ONESHOT

// use std::{thread, time::Duration};

// use tokio::sync::oneshot;

// async fn um_calculo() -> String {
//     thread::sleep(Duration::new(2, 0));
//     "Resultado".to_string()
// }
// #[tokio::main]
// async fn main() {
//     let (tx, rx) = oneshot::channel();
//     tokio::spawn(async move {
//         let res = um_calculo().await;
//         tx.send(res).unwrap();
//     });
//     thread::sleep(Duration::new(1, 0));
//     println!("calcula 1");

//     thread::sleep(Duration::new(2, 0));
//     println!("calculo 2");
//     let res = rx.await.unwrap();
//     println!("{}", res);
// }

// EXEMPLO DE MPSC(Multiples Producers Single consumer )

// use tokio::sync::mpsc;

// // Vamos substituir o #{ por @, mas se formos fazer isso um por um em outro exemplo grande ira demorar muito entao vamos fazer isso com channels para aumentar a perfomace
// const EDN: &'static str = "{
//         :a	#{:otavio	:icaro	:thais	:tanara	:aline	:vanessa}
//         :b	#{:vivi	:rafa	:elisa	:cintia	#{:tami	:tamires}}
//         :c	#{:juily	:camila	:isabela	#{:carol	ricca}}
// }";

// async fn replace_set(edn_str: &str) -> String {
//     edn_str.replace("#{", "@")
// }

// #[tokio::main]
// async fn main() {
//     // 100 é a capacidade total do canal, o numero de valores que ele aguenta como pendentes
//     let (mut tx, mut rx) = mpsc::channel(100);
//     tokio::spawn(async move {
//         for e in EDN.split('\n') {
//             let res = replace_set(e).await;
//             tx.send(res).await.unwrap();
//         }
//     });
//     let mut formatted_edn = String::new();
//     while let Some(res) = rx.recv().await {
//         formatted_edn.push_str(&res);
//     }
//     println!("got = {}", formatted_edn)
// }

// use tokio::sync::{mpsc, oneshot};

// // Exemplo usando ambos
// enum Command {
//     Increment,
// }
// #[tokio::main]
// async fn main() {
//     // Criando dois cainais
//     let (cmd_tx, mut cmd_rx) = mpsc::channel::<(Command, oneshot::Sender<u64>)>(100);

//     // Criando uma thread com tokio
//     tokio::spawn(async move {
//         let mut counter = 0;
//         // Criando uma tupla com o comando e a responta que irei receber do for_each da linha 89
//         while let Some((cmd, response)) = cmd_rx.recv().await {
//             match cmd {
//                 Command::Increment => {
//                     // valor anterior
//                     let prev = counter;
//                     counter += 1;
//                     // aqui estou enviando o valor anterior(do counter), para um canal oneshot que foi criado nas linhas abaixo (o nome é resp_rx)
//                     response.send(prev).unwrap();
//                 }
//             }
//         }
//     });
//     // criando um vetor de threads?
//     let mut join_handlers = vec![];
//     // criando um range de 0 a 9 com um for each
//     (0..10).for_each(|_| {
//         // Fazendo um clone do trasmissor(mpsc) porque cada vez que percorrer o range ele precisa ser dona dela mesma
//         let cmd_tx = cmd_tx.clone();
//         // Iserindoa as threads no vetor join_handlers
//         join_handlers.push(tokio::spawn(async move {
//             // Criando o transmissor e o canal que ira receber os valores (oneshot)
//             let (resp_tx, resp_rx) = oneshot::channel();

//             // fazendo o canal trasmisor(mpsc) mandar para o canal recebidor(o nome é cmd_rx) o comando que ele quer
//             // e tambem esta mandando um canal(oneshot) recebidor que vai receber as respostas(ele se chama response) na thread da linha 71
//             // resp_tx = response da linha 81
//             cmd_tx
//                 .send((Command::Increment, resp_tx))
//                 .await
//                 .ok()
//                 .unwrap();

//             //Recebendo os valores do canal(oneshot que tem o nome de resp_tx) e guardando ela numa variavel
//             let res = resp_rx.await.unwrap();
//             // Pritando os valores
//             println!("Valor antes de incrementar = {}", res);
//         }));
//     });

//     //Acordando as threads?
//     for join_handler in join_handlers.drain(..) {
//         join_handler.await.unwrap();
//     }
// }

use std::{collections::HashMap, io, time::Duration};

use tokio::{
    sync::{broadcast, watch},
    time,
};

// EXEMPLOS COM BROADCAST
//#[tokio::main]
// async fn main() {
//     // Criando o canal
//     let (tx, mut rx1) = broadcast::channel(16);
//     // Inscrevendo o segundo recebidor no canal tx, assim ele tambem podera receber as mensagens de tx
//     let mut rx2 = tx.subscribe();
//     tokio::spawn(async move {
//         assert_eq!(rx1.recv().await.unwrap(), 10);
//         assert_eq!(rx1.recv().await.unwrap(), 20);
//     });

//     tokio::spawn(async move {
//         assert_eq!(rx2.recv().await.unwrap(), 10);
//         assert_eq!(rx2.recv().await.unwrap(), 20);
//     });

//     tx.send(10).unwrap();
//     tx.send(20).unwrap();
// }

// EXEMPLO COM WATCHER
#[derive(Debug, Clone, Eq, PartialEq)]
struct Config {
    timeout: Duration,
    time_between_reviews: Duration,
    state: HashMap<String, String>,
}
impl Config {
    async fn load_from_file() -> io::Result<Config> {
        //	Função	responsável	por	carregar	todos	os	campos	de	con
        // figuração	e	responder	com	um	`Result`	contendo	o	estado	atualizad
        // o	das	configs
    }
    async fn operacao_async() {
        //qualquer tarefa
    }
}
#[tokio::main]
async fn main() {
    // Carregar o estado inicial das configurações
    let mut config = Config::load_from_file().unwrap();

    // criando um canal watch contendo as configurações inicalizadas em config
    let (tx, rx) = watch::channel(config.clone());

    // Criar uma task async que fica em loop e monitora o estado das configs
    tokio::spawn(async move {
        loop {
            //`time::delay`	de `Config.time_between_reviews`.
            //`time_between_reviews` significa tempo	entre	reviões.
            tokio::time::delay_for(Config.time_between_reviews).await;
            //	recarrega	`Config`	em	uma	nova	variável	`config_atualiizada`
            let config_atualiizada = Config::load_from_file().await.unwrap();
            //	Se	a	`config_atualizada`	é	diferente	da	config	existente,
            //	envia	seu	conteúdo	por	mensagem	no	canal	`watch`.
            if config_atualiizada != config {
                tx.broadcast(config_atualiizada.clone()).unwrap();
                config = config_atualiizada;
            }
        }
    });
}
