---
tags:
  - rust
  - rust/concepto
  - rust/match
status: activo
---

# Concepto - Match

## Que es

La construccion principal de Rust para hacer pattern matching.

## Para que sirve

Permite ejecutar logica distinta segun la forma exacta del valor recibido.

## Por que importa

Es una herramienta central para escribir codigo seguro y explicito con `enum`, `Option` y `Result`.

## Como pensar en `match`

No es un `switch` decorado. Es una forma de desestructurar y verificar exhaustividad.

## Cuando usarlo

Cuando quieres manejar todas las variantes de un tipo de forma explicita.

## Error tipico

Usar `_` demasiado pronto y perder parte del beneficio de exhaustividad.

## Enlaces

- [[Capitulo 02 - Juego de adivinanzas]]
- [[Capitulo 06 - Enums y match]]
- [[Concepto - Option]]
- [[Concepto - Result]]
