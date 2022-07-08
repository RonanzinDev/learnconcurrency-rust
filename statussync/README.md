# Sincronização de estado
 
Para executar threads com segurança, muitas vezes precisamos de formas de sincronizar o estado e para isso Tokio dispões de alguns primitivos de sincronização de estado.São primitivos igual ao do std(Sync e Send), mas com habilidade de operar sobre processos assíncronos e, em vez de bloquear a thread, são poperar assincronamente sua liberação. Dependem da feature sync.

### `Barrier`
Garante que múltiplos processos vão esperar uns aos outros para continuar até atingirem certo ponto na execução do programa.

### `Mutex`
Ele garante que no maixo uma thread por vez tenha acesso aos dados. Sua maior diferença com o Mutex que ja vem no std do rust é que os seus locks não bloqueiam e o lockguard pode ser mantido por varios pontos de await. O principal casod e uso de `async Mutex` é prover acesso mutavel e compatilhado a recursos de IO como conexão com banco de dados. Assim, se os recursos que a Mutex armazenam somente dados, é mais vantajoso utilizar as Mutex da std ou de crates como parking_lot, pois o recurso que a `async Mutex` provê, que é manter a Mutex com lock através de varios pontos de await, é raramente usado para dados.


### `Notify`

Prove um simples mecanismos de notificação de eventos para uma unica thread. O notify não carrega nenhum dado com si, mas serve para sinalizar outro processo de que uma ação poder ser perfomada.