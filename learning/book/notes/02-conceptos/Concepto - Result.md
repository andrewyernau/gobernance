---
tags:
  - rust
  - rust/concepto
  - rust/result
status: activo
---

# Concepto - Result

## Que es

Un `enum` que representa exito o error: `Ok(T)` o `Err(E)`.

## Para que sirve

Obliga a tratar errores recuperables de forma explicita.

## Por que importa

Rust no quiere que ignores facilmente los errores que pueden ocurrir en tiempo de ejecucion.

## Como usarlo

`parse()`, lectura de archivos y muchas operaciones del sistema devuelven `Result`.

## Cuando usarlo

Cuando una operacion puede fallar y el programa todavia puede decidir que hacer.

## Error tipico

Abusar de `unwrap()` sin pensar si ese fallo realmente deberia ser un panic.

## Enlaces

- [[Capitulo 02 - Juego de adivinanzas]]
- [[Concepto - Match]]
- [[Concepto - Option]]
