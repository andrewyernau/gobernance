---
tags:
  - rust
  - rust/concepto
  - rust/if-let
status: activo
---

# Concepto - if let y let else

## Que es

Dos formas concisas de hacer pattern matching cuando no necesitas un `match` completo.

## Para que sirve

`if let` se centra en un caso relevante e ignora el resto. `let...else` mantiene el camino feliz limpio cuando el caso que falla debe salir pronto.

## Por que importa

Te ayuda a escribir menos ruido sin perder el modelado expresivo de enums, `Option` y `Result`.

## Cuando usarlo

Usa `if let` cuando solo quieres ejecutar algo si un patron coincide. Usa `let...else` cuando necesitas extraer un valor o abortar pronto.

## Error tipico

Usarlo donde `match` deberia seguir siendo exhaustivo, o escribir un `else` que no corta realmente el flujo en `let...else`.

## Enlaces

- [[Capitulo 06 - Enums y match]]
- [[Concepto - Match]]
- [[Concepto - Option]]
