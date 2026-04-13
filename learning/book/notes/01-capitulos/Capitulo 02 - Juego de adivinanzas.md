---
tags:
  - rust
  - rust/book
  - rust/ch02
status: completado
---

# Capitulo 02 - Juego de adivinanzas

## Idea central

Construir un programa pequeño con entrada por teclado, parseo de datos, comparacion y una dependencia externa.

## Que introduces por primera vez

- [[Concepto - Cargo]] con dependencias
- [[Concepto - Variables y mutabilidad]]
- [[Concepto - Match]]
- [[Concepto - Result]]

## Codigo del repo

- `learning/book/ch02/guessing_game`

## Por que importa

Este capitulo ya se parece a software real: hay input, flujo, errores y una libreria externa.

## Error tipico

Pensar que `parse()` devuelve directamente el numero. No: devuelve `Result`, porque la conversion puede fallar.

## Enlaces

- [[Capitulo 03 - Conceptos comunes]]
- [[Concepto - Result]]
