# Gobernance Rust Workspace

Este repositorio queda dividido en dos líneas de trabajo:

- `learning/`: progreso estructurado del libro oficial de Rust.
- `games/sandbox/`: base del futuro juego tipo sandbox.

## Estructura

- `learning/book/ch01` a `learning/book/ch04`: ejercicios y ejemplos alineados con `book.rustlang-es.org`.
- `games/sandbox`: crate inicial del proyecto de juego.
- `tools/get-dependencies`: crate utilitario para comprobar dependencias o hacer pruebas rápidas.
- `docs/rustbook.md`: mapa de referencia oficial y relación entre capítulos y código.

## Flujo recomendado

1. Lee el capítulo en `https://book.rustlang-es.org/`.
2. Revisa el `README.md` del capítulo correspondiente.
3. Ejecuta el ejemplo Rust asociado.
4. Registra el aprendizaje en el vault de Obsidian.

## Comandos útiles

```powershell
cargo check --workspace
cargo run -p hello_cargo
cargo run -p guessing_game
cargo run -p sandbox_game
```

## Decisión de organización

Se mantiene el código en `gobernance` y el conocimiento en `VAULT`, pero ambos quedan conectados por documentación espejo:

- en el repo, para explicar código y objetivos;
- en Obsidian, para crear memoria, nodos y seguimiento del aprendizaje.

