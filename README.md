# Gobernance Rust Workspace

Este repositorio queda dividido en dos líneas de trabajo:

- `learning/`: progreso estructurado del libro oficial de Rust.
- `games/cubix/`: base del futuro juego tipo sandbox.

## Estructura

- `learning/book/`: ejemplos por capítulo y vault de notas enlazadas para Obsidian.
- `learning/book/notes/`: mapas, capítulos y conceptos conectados con wikilinks.
- `games/cubix`: crate inicial del proyecto de juego.
- `tools/get-dependencies`: crate utilitario para comprobar dependencias o hacer pruebas rápidas.
- `docs/rustbook.md`: mapa de referencia oficial y relación entre capítulos y código.

## Flujo recomendado

1. Lee el capítulo en `https://book.rustlang-es.org/`.
2. Abre `learning/book/notes/00-mapas/Rust Book - Hub.md` en Obsidian.
3. Entra al capítulo o concepto enlazado.
4. Ejecuta el ejemplo Rust asociado.
5. Añade tus propias notas encima de la red existente.

## Comandos útiles

```powershell
cargo check --workspace
cargo run -p hello_cargo
cargo run -p guessing_game
cargo run -p cubix
```

## Decisión de organización

Se mantiene el código y el conocimiento dentro del mismo repositorio, pero separados por capas:

- `learning/book/*`: código ejecutable y ejemplos.
- `learning/book/notes/*`: conocimiento atómico para Obsidian.
- `games/cubix/*`: objetivo práctico de largo plazo.
