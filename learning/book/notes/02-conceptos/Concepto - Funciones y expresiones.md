---
tags:
  - rust
  - rust/concepto
  - rust/funciones
status: activo
---

# Concepto - Funciones y expresiones

## Que es

Rust separa muy claramente sentencias y expresiones. Muchas construcciones devuelven valor.

## Para que sirve

Hace posible escribir codigo compacto sin perder claridad, y ayuda a razonar sobre retornos.

## Por que importa

Si no entiendes que un bloque puede devolver un valor, muchas APIs de Rust se sienten raras.

## Como reconocerlo

```rust
fn square(x: i32) -> i32 {
    x * x
}
```

No hay `;` al final porque la ultima expresion se retorna.

## Error tipico

Poner `;` donde querias devolver algo, convirtiendo la expresion en sentencia.

## Enlaces

- [[Capitulo 03 - Conceptos comunes]]
- [[Concepto - Flujo de control]]
