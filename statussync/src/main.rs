use std::sync::Arc;
use tokio::sync::Notify;
// EXEMPLO COM BARRIER

// #[tokio::main]
// async fn main() {
//     let mut handles = Vec::with_capacity(10);
//     let barrier = Arc::new(Barrier::new(10));
//     for i in 0..10 {
//         let c = barrier.clone();
//         // As mensagens sera exibidas juntas.
//         // Não haverá mensagens intercaladas
//         handles.push(async move {
//             println!("antes do wait {}", i);
//             let wr = c.wait().await;
//             println!("depois do wait {}", i);
//             wr
//         });
//     }
//     // Não resolvera até que todas as mensagens "antes do wait" sejam exibidas
//     let wrs = join_all(handles).await;
//     // Somente um barrier vai ser resolvido como líder, ou em ingles "leader"
//     assert_eq!(wrs.into_iter().filter(|wr| wr.is_leader()).count(), 1);
// }

// EXEMPLO COM MUTEX DO TOKIO
// #[tokio::main]
// async fn main() {
//     // Envolvendo a mutex em um Arc para permitir que ela seja compartilhada por várias threads
//     let count = Arc::new(Mutex::new(0));
//     for _ in 0..5 {
//         let my_count = Arc::clone(&count);
//         tokio::spawn(async move {
//             for _ in 0..10 {
//                 let mut lock = my_count.lock().await;
//                 *lock += 1;
//                 println!("{}", lock);
//             }
//         });
//     }
//     loop {
//         if *count.lock().await >= 50 {
//             break;
//         }
//     }
//     println!("Contagem	atingiu	50.");
// }

// EXEMPLO COM NOTIFY
#[tokio::main]
async fn main() {
    let notify = Arc::new(Notify::new());
    let notify2 = notify.clone();
    tokio::spawn(async move {
        notify2.notified().await;
        println!("Recebeu notificação");
    });
    println!("enviou notificação");
    notify.notify_one();
}
