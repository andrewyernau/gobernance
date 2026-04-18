---
tags:
  - rust
  - rust/concepto
  - rust/vec
status: activo
---

# Concepto - Vec

## Que es

La coleccion dinamica `Vec<T>`, pensada para almacenar una secuencia de valores del mismo tipo en memoria contigua.

## Para que sirve

Es la estructura base cuando necesitas una lista que puede crecer y sobre la que iteraras mucho.

## Por que importa

En Rust, `Vec` aparece enseguida en datos reales: bloques, vertices, entidades, comandos, resultados y buffers.

## Como usarlo

Crea con `vec![]` o `Vec::new()`, anade con `push`, accede con `get` si quieres seguridad explicita e itera con `for`.

## Cuando usarlo

Cuando el orden importa y el acceso por indice o la iteracion son mas naturales que la busqueda por clave.

## Error tipico

Confiar demasiado en la indexacion directa y olvidar que insertar o borrar puede invalidar referencias existentes.

## Enlaces

- [[Capitulo 08 - Colecciones comunes]]
- [[Concepto - Ownership]]
- [[Concepto - Borrowing y referencias]]
- [[Proyecto - Sandbox con Vulkan]]
