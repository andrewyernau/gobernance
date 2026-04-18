# Capitulo 6 - Enums y pattern matching

## Fuente oficial

- https://book.rustlang-es.org/ch06-00-enums
- https://book.rustlang-es.org/ch06-01-defining-an-enum
- https://book.rustlang-es.org/ch06-02-match
- https://book.rustlang-es.org/ch06-03-if-let

## Codigo

- `enums/`: definicion de enums y modelado de variantes.
- `matches/`: uso de `match` para comportamiento segun la variante.
- `iflet/`: `if let`, `if let ... else` y `let...else` para mantener limpio el camino feliz.

## Orden recomendado

1. `enums`
2. `matches`
3. `iflet`

## Utilidad

- Modelar estados cerrados de forma segura.
- Expresar comportamiento distinto segun el tipo exacto de valor.
- Preparar el terreno para bloques, eventos, inputs y estados del juego.

## Comandos utiles

```powershell
cargo run --manifest-path ch06/enums/Cargo.toml
cargo run --manifest-path ch06/matches/Cargo.toml
cargo run --manifest-path ch06/iflet/Cargo.toml
```

## Nota en Obsidian

- `../notes/01-capitulos/Capitulo 06 - Enums y match.md`
