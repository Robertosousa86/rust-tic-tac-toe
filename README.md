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

### Solução

O tipo que se encaixa nessa solução é o `enum`, com algumas `derivações`

```rust
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Player {
    X,
    O,
}
```

Mas... Qual o significado desse trecho e código?

- **Derive**
  \
   O atributo `derive` permite que novos itens sejam gerados automaticamente para estruturas de dados. Ele usa a sintaxe `MetaListPaths` para especificar uma lista de características a serem implementadas ou possibilidades de derivar `macros` para serem processadas. No exemplo abaixo iremos fazer uma implementação para as `traits` `PartialEq` e `Clone` com a intenção de serem utilizadas em `Foo`, e o parâmetro de tipo `T` receberá as `constraints` PartialEq ou Clone para a `impl` apropriada:

```rust
  #[derive(PartialEq, Clone)]
  struct Foo<T> {
    a: i32,
    b: T,
  }
```

A implementação (`impl`) gerada para `PartialEq` é o equivalente a:

```rust
impl<T: PartialEq> PartialEq for Foo<T> {
  fn eq(&self, other: &Foo<T>) -> bool {
    self.a == other.a && self.b == other.b
  }

  fn ne(&self, other: &Foo<T>) -> bool {
    self.a != other.a || self.b != other.b
  }
}
```

- **Debug**
  \
   Gera uma representação do tipo `string` para o debug, que pode ser usada através da `macro` `dbg!()`;

- **Eq** e **PartialEq**
  \
  Dizem ao compilador que ele pode comparar a igualdade de tipos utilizando `==` e `!=`;

- **Copy** e **Clone**
  \
  Diz ao compilador que ele é livre para copiar o tipo por todo lugar, isso significa que nós não precisaremos nos preocupar com a semântica de movimento por enquanto.
