---
tags:
  - rust
  - rust/book
  - rust/ch06
status: en_progreso
---

# Capitulo 06 - Enums y match

## Estado

Progreso actual: hasta `6.2 match`.

## Idea central

Modelar estados cerrados y ejecutar comportamiento distinto segun la variante exacta de un valor.

## Subcapitulos

- 6.1 Definiendo un enum -> [[Concepto - Enums]]
- 6.2 `match` -> [[Concepto - Match]]
- 6.3 `if let` y `let else` -> siguiente paso

## Codigo del repo

- `learning/book/ch06/enums`
- `learning/book/ch06/matches`

## Por que importa

Enums + `match` son una de las herramientas mas potentes de Rust para modelar:

- tipos de bloque;
- estados del jugador;
- eventos de input;
- resultados de sistemas internos.

## Señal de dominio

Cuando un `enum` te parece mas natural que un conjunto de strings o numeros magicos, vas bien.

## Enlaces

- [[Capitulo 05 - Structs]]
- [[Concepto - Enums]]
- [[Concepto - Match]]
- [[Concepto - Option]]
- [[Proyecto - Sandbox con Vulkan]]
