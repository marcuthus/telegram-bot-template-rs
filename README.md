# Telegram Bot Template (Rust) 🦀

Um template robusto e moderno para criação de bots de Telegram utilizando **Rust**, **Teloxide** e as melhores práticas de desenvolvimento. Inspirado no [telegram-bot-template](../telegram-bot) em TypeScript.

## ✨ Funcionalidades

- 🦀 **Rust + Tokio**: tipagem forte, segurança de memória e performance.
- ⚡ **Comandos com Teloxide**: enum `Command` com `#[derive(BotCommands)]`, argumentos tipados e menu sincronizado com o Telegram.
- 📂 **Organização modular**: cada categoria de comando e cada evento vive no seu próprio módulo.
- 📝 **Logs com `tracing`**: console colorido e arquivos em `logs/` (`combined.log` e `error.log`).
- 🎨 **Qualidade de código**: `rustfmt` (4 espaços) e `clippy` com `-D warnings`, validados no CI.
- ⚙️ **Gestão de variáveis**: `.env` via `dotenvy`, validado na inicialização com erro claro quando falta alguma variável.
- 🔐 **Checks de permissão**: exemplo de comando restrito ao dono do bot (`/uptime`).

## 🛠️ Pré-requisitos

- [Rust](https://www.rust-lang.org/tools/install) (edition 2024, Rust 1.85 ou superior)
- Um bot criado com o [@BotFather](https://t.me/BotFather)

## 🚀 Instalação e Início

1.  **Clone o repositório:**

    ```bash
    git clone https://github.com/marcuthus/telegram-bot-rs.git
    cd telegram-bot-rs
    ```

2.  **Configure o ambiente:**
    Copie o arquivo `.env.example` para `.env` e preencha as variáveis:

    ```env
    TELEGRAM_BOT_TOKEN=token_do_botfather
    TELEGRAM_ADMIN_ID=seu_id_de_usuario_telegram
    ```

3.  **Inicie o bot:**

    ```bash
    # Modo desenvolvimento
    cargo run

    # Build para produção
    cargo build --release
    ./target/release/telegram-bot-rs
    ```

## 📜 Comandos Úteis

| Comando                                       | Descrição                                 |
| --------------------------------------------- | ----------------------------------------- |
| `cargo run`                                   | Executa o bot em modo de desenvolvimento. |
| `cargo build --release`                       | Compila o bot otimizado para produção.    |
| `cargo fmt`                                   | Formata o código.                         |
| `cargo clippy --all-targets -- -D warnings`   | Verifica erros de linting.                |
| `cargo test`                                  | Executa os testes.                        |

Defina `RUST_LOG=debug` para ver logs mais detalhados no console.

## 📁 Estrutura de Pastas

```text
src/
├── main.rs          # Ponto de entrada
├── client.rs        # Criação do bot e do dispatcher
├── data.rs          # Data, Context e Error compartilhados
├── error.rs         # Handler global de erros
├── commands/        # Enum Command e implementação por categoria
│   ├── info/
│   ├── example/
│   └── admin/
├── events/          # Handlers de eventos do Telegram
├── helpers/         # Auxiliares (config e logger)
│   ├── config.rs    # Variáveis de ambiente (Config)
│   └── logger.rs    # Configuração do tracing (console + arquivos)
└── utils/           # Utilitários (checks de permissão, etc.)
```

## ➕ Adicionando um comando

1. Adicione uma variante ao enum `Command` em `src/commands/mod.rs` com `#[command(description = "...")]` (os campos da variante são os argumentos).
2. Crie `src/commands/<categoria>/<comando>.rs` com `pub async fn run(ctx: &Context, ...)` e declare o módulo no `mod.rs` da categoria.
3. Despache a variante no `match` de `commands::run` e adicione o nome em `Command::name`.

Os comandos são sincronizados com o menu do Telegram na inicialização (`set_my_commands`).

> Diferente do template em TypeScript, Rust não carrega módulos em tempo de execução, e a validação de argumentos é feita pelo próprio Teloxide em vez do Argus.

## ➕ Tratando um novo evento

Crie `src/events/<evento>.rs` e registre um novo ramo no handler `dptree` em `src/client.rs`.

## ⚖️ Licença

Este projeto está sob a licença MIT. Veja o arquivo [LICENSE](LICENSE) para mais detalhes.

---

Feito com ❤️ por [Marcuth](https://github.com/marcuth)
