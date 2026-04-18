# Capitulo 9 - Manejo de errores

## Fuente oficial

- https://book.rustlang-es.org/ch09-00-error-handling
- https://book.rustlang-es.org/ch09-01-unrecoverable-errors-with-panic
- https://book.rustlang-es.org/ch09-02-recoverable-errors-with-result
- https://book.rustlang-es.org/ch09-03-to-panic-or-not-to-panic

## Codigo

- `src/main.rs`: diferencia entre invariantes que merecen `panic!`, errores recuperables con `Result` y propagacion con `?`.

## Orden recomendado

1. Lee `main` para ver primero la decision de alto nivel: que se recupera y que no.
2. Recorre `parse_render_distance` y `load_spawn_height` para ver `Result` y `?`.
3. Cierra con `require_valid_chunk_size` para fijar cuando un `panic!` es razonable.

## Utilidad

- Preparar la carga de configuracion, assets y shaders sin llenar el codigo de `unwrap()`.
- Entender por que `vulkanalia` y `anyhow::Result` encajan tan bien en aplicaciones reales.
- Decidir donde fallar rapido y donde devolver contexto al caller.

## Comandos utiles

```powershell
cargo run --manifest-path ch09/Cargo.toml
$env:RUST_BACKTRACE=1; cargo run --manifest-path ch09/Cargo.toml
```

## Nota en Obsidian

- `../notes/01-capitulos/Capitulo 09 - Manejo de errores.md`
