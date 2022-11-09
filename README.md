# Jogo da velha em Rust (Tic Tac Toe)

## O clássico jogo da velha criado em Rust

Esse projeto é baseado no conteúdo `RS118_UWCS Learn Rust` e é divido em tarefas. As tarefas podem ir desde criar o projeto propriamente com o comando `cargo new` até pesquisas no google.

- [x] Task 0: Criar um novo Projeto

### Solução

Vamos criar um novo projeto na pasta `Projetos` utilizando o comando `cargo new tic_tac_toe`. Depois usamos o comando `cd` para entrar no diretório criado (tic_tac_toe) e por fim `code .` para abrir o `Visual Studio Code` (editor escolhido, mas não obrigatório) na raiz do projeto.

`Cargo` é o gerenciador de pacotes do Rust. O `Cargo` faz downloads das dependências, compila seus pacotes, cria pacotes distribuíveis e faz o upload deles para o `creates.io`, o registro de pacotes da comunidade Rust.

- [x] Task 1: Tipos de dados

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

- Enum
  \
  Em Rust, um `enum` é uma estrutura de dados que declara seus diferentes tipos. Um `enum` prove as mesmas funcionalidades de uma `struct`, mas utiliza menos código. Por exemplo, implementando diferentes tipos de `Maquina`, cada `maquina` tera tipos diferente de atributos e requerem diferentes `struct's` para cada `maquina`. Por outro lado, podemos incorporar todos esses diferentes tipos de `maquinas` em uma única `enum`:

```rust
struct Phone {}
struct Adder {
    x: i,
    y: i,
}
struct Computer {
    name: String,
}
```

Além disso, não é possível criar uma função que possa receber qualquer tipo de `Maquina` como parâmetro usando `struct's`; nesse caso, `enum's` devem ser usados. Com um `enum`, o tipo de `maquina` pode ser identificado dentro da função usando a `pattern matching` com a palavra reservada `match`.

```rust
#![allow(unused_variables)]
fn main() {
    #[allow(dead_code)]
    enum Machine {
        Phone,
        Adder(i8, i8),
        Computer(String),
}
// A function taking any Machine as parameter:
fn foo(m: Machine) {
    match m {
        Machine::Phone => println!("This is a phone"),
        Machine::Adder(a, b) => println!("Sum: {}", a + b),
        Machine::Computer(name) => println!("Name: {}", name),
    }
}

let machine_1 = Machine::Phone;
let machine_2 = Machine::Adder(3, 4);
let machine_3 = Machine::Computer("Mainframe".to_string());

foo(machine_1);
foo(machine_2);
foo(machine_3);

}
```

Uma função também pode ser declarada para um `enum` usando a palavra-chave `impl`, que é semelhante a uma `struct`:

```rust
#![allow(unused_variables)]
fn main() {
    #[allow(dead_code)]
    enum Machine {
        Phone,
        Adder(i8, i8),
        Computer(String),
}

impl Machine{
    fn foo(&self) {
        match self {
            Machine::Phone => println!("This is a phone"),
            Machine::Adder(a, b) => println!("Sum: {}", a + b),
            Machine::Computer(name) => println!("Name: {}", name),
        }
    }
}

let machine_1 = Machine::Phone;
let machine_2 = Machine::Adder(3, 4);
let machine_3 = Machine::Computer("Mainframe".to_string());

machine_1.foo();
machine_2.foo();
machine_3.foo();
}
```

**Fonte:** Whats is a enum in Rust? **Disponível em:** [educative](https://www.educative.io/answers/what-is-an-enum-in-rust)

- [] 1.2: Implementar um tipo de dado simples para representar o estado do jogo/tabuleiro
