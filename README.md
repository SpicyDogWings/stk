# stk

Pinta texto en la terminal. Un pequeño CLI en Rust que colorea (y pone en
negrita) el texto que le pases.

## Requisitos

- Rust y Cargo (edition 2024, toolchain reciente)

## Instalación

Desde el código fuente:

    cargo build --release
    # binario en target/release/stk

O instalándolo en tu `PATH`:

    cargo install --path .

## Uso

    stk <TEXT>... [-f <COLOR>]

Ejemplos:

    stk "hola mundo"
    stk "hola" -f red
    stk hola mundo --foreground bright-cyan

## Opciones

| Argumento                    | Descripción                | Default         |
|------------------------------|----------------------------|-----------------|
| `<TEXT>...`                  | Texto a pintar (uno o más) | — (obligatorio) |
| `-f`, `--foreground <COLOR>` | Color del texto            | `green`         |

## Colores disponibles

`black`, `red`, `green`, `blue`, `cyan`, `yellow`, `magenta`, `white`,
`bright-black`, `bright-red`, `bright-green`, `bright-blue`, `bright-cyan`,
`bright-yellow`, `bright-magenta`, `bright-white`.

## Cómo funciona

- `clap` parsea los argumentos (`text` y `foreground`).
- `anstyle` construye el estilo ANSI: color de frente + negrita (`Effects::BOLD`).
- `StkColor` se traduce a `anstyle::Color` con `impl From<StkColor> for Color`.

## Desarrollo

    cargo build
    cargo run -- "texto" -f green
    cargo fmt
    cargo clippy

## Licencia

Este proyecto se distribuye bajo la licencia MIT. Ver el archivo [LICENSE](LICENSE).
