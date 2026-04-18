---
tags:
  - rust
  - rust/concepto
  - rust/option
status: activo
---

# Concepto - Option

## Que es

Un `enum` que representa presencia o ausencia de valor: `Some(T)` o `None`.

## Para que sirve

Evita usar `null` y obliga a contemplar el caso en el que no hay dato.

## Por que importa

La ausencia de valor es parte del dominio. Rust quiere que eso se vea en el tipo.

## Como usarlo

Se combina muy bien con [[Concepto - Match]] y con [[Concepto - if let y let else]].

## Cuando usarlo

Cuando algo puede existir o no sin que eso sea un error.

## Error tipico

Confundir "no hay valor" con "ha ocurrido un error". Si hubo un fallo real, muchas veces lo correcto es `Result`.

## Enlaces

- [[Capitulo 06 - Enums y match]]
- [[Concepto - Enums]]
- [[Concepto - if let y let else]]
- [[Concepto - Result]]
