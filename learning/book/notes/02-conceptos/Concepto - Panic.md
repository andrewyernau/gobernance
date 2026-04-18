---
tags:
  - rust
  - rust/concepto
  - rust/panic
status: activo
---

# Concepto - Panic

## Que es

La parada irrecuperable del programa cuando se detecta un estado imposible o un error que no se puede manejar localmente.

## Para que sirve

Marca fallos de programacion, invariantes rotas o caminos que no deberian poder ocurrir.

## Por que importa

Rust diferencia entre error recuperable y bug. `panic!` existe para no mezclar ambas cosas.

## Como usarlo

Puede aparecer por una llamada explicita a `panic!`, por `unwrap()` y `expect()`, o por errores como indexar fuera de rango.

## Cuando usarlo

Cuando el programa ya no puede seguir correctamente porque una suposicion interna esencial se ha roto.

## Error tipico

Usarlo para fallos esperables de usuario, IO o configuracion en vez de devolver un `Result`.

## Enlaces

- [[Capitulo 09 - Manejo de errores]]
- [[Concepto - Result]]
- [[Concepto - Propagacion de errores]]
- [[Proyecto - Sandbox con Vulkan]]
