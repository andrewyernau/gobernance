# Capitulo 8 - Colecciones comunes

## Fuente oficial

- https://book.rustlang-es.org/ch08-00-common-collections
- https://book.rustlang-es.org/ch08-01-vectors
- https://book.rustlang-es.org/ch08-02-strings
- https://book.rustlang-es.org/ch08-03-hash-maps

## Codigo

- `vectors/`: `Vec<T>`, acceso seguro con `get`, indexacion e iteracion mutable.
- `strings/`: `String`, `&str`, concatenacion y primeras fricciones con UTF-8.
- `hashmaps/`: `HashMap<K, V>`, busqueda por clave y `entry().or_insert()`.

## Orden recomendado

1. `vectors`
2. `strings`
3. `hashmaps`

## Utilidad

- Modelar listas reales del juego: bloques, entidades, vertices y chunks.
- Entender por que `String` no se trata como un array de caracteres.
- Elegir `HashMap` cuando el acceso por clave importa mas que el orden.
- Ver ownership y borrowing trabajando dentro de estructuras de datos reales.

## Comandos utiles

```powershell
cargo run --manifest-path ch08/vectors/Cargo.toml
cargo run --manifest-path ch08/strings/Cargo.toml
cargo run --manifest-path ch08/hashmaps/Cargo.toml
```

## Nota en Obsidian

- `../notes/01-capitulos/Capitulo 08 - Colecciones comunes.md`
