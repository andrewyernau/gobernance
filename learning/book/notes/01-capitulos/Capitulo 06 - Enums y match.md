---
tags:
  - rust
  - rust/book
  - rust/ch06
status: completado
---

# Capitulo 06 - Enums y match

## Idea central

Modelar estados cerrados y ejecutar comportamiento distinto segun la variante exacta de un valor.

## Subcapitulos

- 6.1 Definiendo un enum -> [[Concepto - Enums]]
- 6.2 `match` -> [[Concepto - Match]]
- 6.3 `if let` y `let else` -> [[Concepto - if let y let else]]

## Codigo del repo

- `learning/book/ch06/enums`
- `learning/book/ch06/matches`
- `learning/book/ch06/iflet`

## Ruta recomendada

1. Primero modela el dominio con `enum`.
2. Usa `match` cuando necesitas exhaustividad real.
3. Pasa a `if let` o `let...else` cuando solo te interesa un caso y quieres dejar limpio el happy path.

## Por que importa

Enums + `match` son una de las herramientas mas potentes de Rust para modelar:

- tipos de bloque;
- estados del jugador;
- eventos de input;
- resultados de sistemas internos.

## Error tipico

- usar `_` demasiado pronto y perder exhaustividad;
- forzar `match` cuando `if let` o `let...else` harian el flujo mas legible;
- modelar estados cerrados con strings o enteros magicos.

## Enlaces

- [[Capitulo 05 - Structs]]
- [[Capitulo 07 - Paquetes, crates y modulos]]
- [[Concepto - Enums]]
- [[Concepto - Match]]
- [[Concepto - Option]]
- [[Concepto - if let y let else]]
- [[Proyecto - Sandbox con Vulkan]]
