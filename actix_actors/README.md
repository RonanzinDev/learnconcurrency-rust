# Atores com Actix

Actix é um framework de sitemas de atores em Rust. De forma resumida, atoes são como processos que encapsulam estado e comportamento e se comunicam exclusivamente por mensagens. No caso do actix, são mensagens tipidas.

Os atores do Actix são implementados sobre o Tokio e múltiplos atores podem ser executados na mesma thread.As APIs de Arbiter (algo como orquestrador) do Actix permitem que múltiplas threads sejam executadas.Algumas de suas features são:
* Atores Async/Sync.
* Comunicação de atores em um contexto local ou de thread.
* Utiliza futures para gerenciar mensagens	assíncronas.
* Supervisão
* Mensagens	tipadas.

É Importante entender o ciclo de vida de um ator, pois é parte fundamental de seu uso, apesar de muito ja estar abstraído:
* 1 - `started` ou iniciado: uma ator sempre inicia no estado `started`. Nele, o método started() é chamado, porém a trai Actor ja provê uma implementação default para este método. O contexto do ator esá disponivel e permite iniciar outros atores ou registrar streams assíncronas
* 2 - `running` ou executando: uma vez iniciado, o ator transita para o estado running e pode permanecer neste estado indefinidamente.
* 3 - `stopping ou parando`: indica que o ator vai parar de executar e pode acontecer quando própio atore chama o `Context::stop`. Todos os endereços do ator são drop e nenhum evento está registradp mp contexto. Além disso, `Running::Continue` pode reiniciar o ator.
* 4 - `stopped` ou parado: se durante o estado de stopping nenhuma ação é tomada para reiniciar o ator, ele entra no estado `stopped` e tem seu endereço drop.


Nesse projeto, em vez de utilizarmos Tokio par genrenciar nosso runtime async, vamos utilizar o actix-rx que utiliza o Tokio por baixo dos panos

A forma de inciar um ator depende do `Context` que foi declarada na trait Actor; nesse caso utilizamos um contexto do tipo `Context<T>`, que é baseado em futrue e Tokio. Este tipo de contexto pode ser iniciado com Acotr::start() ou Actor::create(). O primeiro é utilizado para criar o ator imediatamente, enquando o segundo é utilizado caso seja preciso acessar o context do ator antes de inicia-lo. Neste momento vamos utulizar somente o start. Tanto start quanto create retorna o endereço do ator e toda comunicação é feita através de `send`, que aguarda o retorno, ou `do_send`, que não aguardo o retorno