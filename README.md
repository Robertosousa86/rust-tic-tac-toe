# Jogo da velha em Rust (Tic Tac Toe)

## O clássico jogo da velha criado em Rust

Esse projeto é baseado no conteúdo `RS118_UWCS Learn Rust` e é divido em tarefas. As tarefas podem ir desde criar o projeto propriamente com o comando `cargo new` até pesquisas no google.

- [x] Task 0: Criar um novo Projeto

### Solução

Vamos criar um novo projeto na pasta `Projetos` utilizando o comando `cargo new tic_tac_toe`. Depois usamos o comando `cd` para entrar no diretório criado (tic_tac_toe) e por fim `code .` para abrir o `Visual Studio Code` (editor escolhido, mas não obrigatório) na raiz do projeto.

`Cargo` é o gerenciador de pacotes do Rust. O `Cargo` faz downloads das dependências, compila seus pacotes, cria pacotes distribuíveis e faz o upload deles para o `creates.io`, o registro de pacotes da comunidade Rust.

- [] Task 1: Tipos de dados

Vamos precisar de alguns tipos de dados para representar o nosso game. Em um jogo com dois jogadores e um tabuleiro, precisaremos representar:

- Dois jogadores, `X` e `O`;
- O tabuleiro é um grid `3x3`, que pode conter: `X`, `O` ou `espaço em branco`.
