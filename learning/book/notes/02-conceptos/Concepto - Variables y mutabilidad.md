---
tags:
  - rust
  - rust/concepto
  - rust/mutabilidad
status: activo
---

# Concepto - Variables y mutabilidad

## Que es

En Rust, las variables son inmutables por defecto. Solo cambian si las declaras con `mut`.

## Para que sirve

Obliga a dejar claro que datos pueden cambiar y cuales no.

## Por que existe

La mutabilidad es una fuente comun de bugs. Rust la hace explicita para reducir ambiguedad.

## Como usarlo

```rust
let x = 5;
let mut y = 10;
y += 1;
```

## Cuando usar `shadowing`

Cuando quieres transformar un valor sin mantener la misma mutabilidad ni el mismo tipo.

## Error tipico

Usar `mut` por costumbre. Mejor empezar inmutable y abrir mutabilidad solo donde aporte valor.

## Enlaces

- [[Capitulo 03 - Conceptos comunes]]
- [[Concepto - Ownership]]
