# Iron

Nosso projeto ira se basear em um banco de dados sobre Dragon Ball, que poderá operar de forma concorrente. Como o banco de dados não é o objetivo nesse momento, vamos contornar isso com outras formas mais rudimentares. Nossos passos serão:

1. Subir uma aplicação Iron baseado em Hello world.
2. Configurar threads e timeouts para o nosso servidor;
3. Aplicar conceitos basicos de rotas
4. Explicar mais a fundo o Iron
5. Pensar em testes para o Iron
6. Criar uma aplicação web basica

## Sobre o Iron
O	Iron	é	um framework de	 servidor	 baseado	 em	middleware com	desempenho	rápido	e	flexível,	que	fornece	uma	base	pequena (mas	 robusta)	 para	 criar	 aplicativos complexos	e RESTful	 APIs.Nenhum	 middleware	 é	 empacotado	 com	 o	 Iron;	 em	 vez	 disso, tudo	 é	drag	 and	 drop	 (arraste	 e	 solte),	 permitindo	 configurações ridiculamente	modulares.	Infelizmente,	o	Iron	não	possui	uma	boa documentação, então, este	 capítulo pode ser uma boa fonte de informações.

