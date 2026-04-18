# Capitulo 7 - Paquetes, crates y modulos

## Fuente oficial

- https://book.rustlang-es.org/ch07-00-managing-growing-projects-with-packages-crates-and-modules
- https://book.rustlang-es.org/ch07-01-packages-and-crates
- https://book.rustlang-es.org/ch07-02-defining-modules-to-control-scope-and-privacy
- https://book.rustlang-es.org/ch07-03-paths-for-referring-to-an-item-in-the-module-tree
- https://book.rustlang-es.org/ch07-04-bringing-paths-into-scope-with-the-use-keyword
- https://book.rustlang-es.org/ch07-05-separating-modules-into-different-files

## Codigo

- `src/lib.rs`: crate de libreria del paquete, con privacidad, `pub use`, rutas absolutas y relativas.
- `src/main.rs`: crate binario que consume la API publica del mismo paquete.
- `src/front_of_house.rs`: modulo separado del archivo raiz del crate.
- `src/front_of_house/hosting.rs`: submodulo en archivo propio para reflejar el arbol de modulos.

## Orden recomendado

1. Lee `Cargo.toml` y asocia paquete con los crates que puede generar.
2. Recorre `src/lib.rs` para entender `mod`, `pub`, `crate::`, `super::` y `pub use`.
3. Salta a `src/front_of_house.rs` y `src/front_of_house/hosting.rs` para ver la separacion fisica del modulo.
4. Cierra en `src/main.rs` para ver como el binario usa solo la interfaz publica.

## Utilidad

- Separar implementacion interna de la API que expones.
- Evitar que el proyecto se degrade en un `main.rs` gigante.
- Preparar el salto a colecciones, errores y testing con una base mantenible.

## Comandos utiles

```powershell
cargo run --manifest-path ch07/Cargo.toml
cargo test --manifest-path ch07/Cargo.toml
```

## Nota en Obsidian

- `../notes/01-capitulos/Capitulo 07 - Paquetes, crates y modulos.md`
