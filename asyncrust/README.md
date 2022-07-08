## Futures

Futures são valores cuja computação talvez ainda não tenha terminado. Esse tipo de "valor assincrono" permite que uma thread continue executando outras tarefa enquando espera o valor se tornar disponivel.

Implementação da trait Future:

<br\>

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;
}
```
<br\>

O metodo que vai definir uma Future é o poll, que vai tentar trasnformar o estado da Future em um valor final definido em Output. O contexto(Context) passado para o metodo pode conter informações sobre como acordad a task.

Exemplos de implemetação de Future na biblioteca naomijub/edn-rs:

<br\>

```rust
use futures::task;
use futures::task::Poll;
use core::pin::Pin

impl futures::future::Future for Edn {
    type Output = Edn;
    fn poll(self: Pin<&mut Self>, cx: &mut task::Context) -> Poll <Self::Output> {
        if !self.to_string().is_empty() {
            let pinned = self.to_owned();
            Poll::Ready(pinned)
        } else {
            Poll::Pending
        }
    }
}
```

Vamos de explicação. Primeira coisa foi definir o `Output = Edn` que sera o nosso retorno e depois a definição do metodo poll. O metodo recebe um `Pin<&mut Self>` e um contexto `&mut task::Context` para retornar um `Poll<Self::Output>`. A trait `Pin` garante que um ponteiro para um ponteiro(foda kkklkk) de qualquer tipo T terá uma posição estável na memoria, que o impediria de ser movido para outra posição e impedira que a memória seja desalocada até que seja drop. 
`Context` nos da acesso a função waker(que traduzido significa despertar) e Poll é um enum com tipos `Ready<Self::Output>`, que significa que o poll es´ta pronto, e Pending, que significa que o poll ainda não está pronto. No caso de `Edn`, o Poll somente será ready qyando conseguirmos converter o Pin para uma string não vazia(o if e else no codigo acima), caso o resultado do if else seja verdadeiro, extraimos o valor de Pin com `let pinned = self.to_owned()` e adicionamos pinned como vlaor em Poll::Ready(pinned).

## Runtime tokio

Tokio é um runtime guiado a evente e possui uma plataforma de IO não bloqueante. E tem alguns componentes como:
* Ferramentas para tarefas assincronas, como primitivos para a sincronizaçao, canais, timeout, atrasos(delay) e intervalos
* Api para executar IO de forma assincrona, incluindo sokets TCP e UDP, operações no sistema de arquivos, gerenciamento de processos.

### Runtime para rust

Os programas assincronos em rust precisam de runtime para prover alguns serviços.
* Event loop com suporte IO
* Um agendador para executar as tarefas que utilizam os recursos IO.
* Um timer para orientar o agendados quando ele precisa executar novamente


Assim o runtime de tokio agregas esses seviços como um tipo unico, permitindo que eles iniciem, desliguem e se configurem conjutamente. A macro `#[tokio::main]` cria um runtime pré-configurada.


### Canais em tokio

A comunicação de tarefas(tasks) é feita através de canais(Channels). Estes canais são basicamente formas de uma tarefa produizr uma mensagem para outra consumir e Tokio nos permite fazer isso de algumas formas. Por exemplo, um canal que suporte multiplos produtores permite que muitas tarefas enviem mensagens, enquanto um canal que suporta multiplos consumidores permite que muitas tarefas recebam mensagens.

#### `OneShot`
O tipo de canal mais basico é o oneshot, que suporta um unico valor de uma unica tarefa produtora para uma unica tarefa consumidora. Este canal é geramente utilizado para enviar a resposta de uma calcula para uma tarefa que espera seu valor.

#### `MPSC`
Multiplos produtore e um unico consumirdo

#### `Broadcast`
Esse canal permite enviar muitos produtores para muitos consumidores,e cada um dos consumidores vai receber todas as mensagens. Ele funciona assim, criamos a famosa dupla tx, rx assim `let (tx, rx) = broadcast::chanel()`, que nos garante que toda mensagem de tx será enviada a pelo menos um rx, e no decorrer do programa pode fazer um sbscribe de um rx qualquer em um tx.


#### `Watcher`
O canal `watch` nos permite enviar muitas mensagens de um unico produtor para diversos consumidores, porem somente a mais atual é armazenada no canal, assim os consumidores são notificados quando uma nova mensagem é enviada, mas não há garantias de que eles vão ver todas as novas mensagens.Casos de uso para o canal watch são geralmente restritos a comunicar atualizações de configurações ou atualizações de estado do programa,como comunicar um shutdown ou um restart.