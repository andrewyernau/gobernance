---
tags:
  - rust
  - rust/proyecto
  - vulkan
  - gamedev
status: activo
---

# Proyecto - Sandbox con Vulkan

## Meta

Construir un juego tipo sandbox, estilo Minecraft, usando Rust y Vulkan.

## Implementacion actual

- `games/cubix`
- [README del crate actual](../../../../games/cubix/README.md)

## Lo que esto exige del lenguaje

- modelado claro de estado con [[Concepto - Structs]] y [[Concepto - Enums]];
- control estricto de memoria con [[Concepto - Ownership]] y [[Concepto - Borrowing y referencias]];
- datos en colecciones y buffers, que apareceran fuerte desde el capitulo 8;
- abstracciones limpias con traits y genericos a partir del capitulo 10;
- concurrencia para carga de mundo, tareas y streaming en los capitulos 16 y 17.

## Traduccion temprana a dominio

Algunos tipos que acabaré necesitando:

- `BlockType` como `enum`
- `Block` como `struct`
- `Chunk` como `struct`
- `World` como `struct`
- `Player` como `struct`
- `GameEvent` e `InputAction` como `enum`

## Lo que debo dominar antes de Vulkan serio

- [[Capitulo 04 - Ownership]]
- [[Capitulo 05 - Structs]]
- [[Capitulo 06 - Enums y match]]
- capitulo 7 para modulos
- capitulo 8 para `Vec` y `HashMap`
- capitulo 10 para traits y lifetimes

