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
- [[Proyecto - Cubix - Triangulo y bootstrap Vulkan]]

## Estado actual del renderer

- El renderer Vulkan ya muestra un triangulo en pantalla.
- La base actual cubre el bootstrap fuerte del renderer y la sincronizacion principal.
- Ya existe recreacion de swapchain ante `OUT_OF_DATE_KHR` y `SUBOPTIMAL_KHR`.
- Falta todavia cerrar el manejo explicito de `WindowEvent::Resized` como parte del siguiente paso natural.
- La documentacion detallada de este hito esta en [[Proyecto - Cubix - Triangulo y bootstrap Vulkan]].

## Lo que esto exige del lenguaje

- modelado claro de estado con [[Concepto - Structs]] y [[Concepto - Enums]];
- control estricto de memoria con [[Concepto - Ownership]] y [[Concepto - Borrowing y referencias]];
- organizacion sana del codigo con [[Concepto - Modulos]] y [[Concepto - Paths y use]];
- datos en colecciones y buffers, que aterrizan en [[Capitulo 08 - Colecciones comunes]];
- manejo de fallos con [[Concepto - Result]], [[Concepto - Panic]] y [[Concepto - Propagacion de errores]], clave desde [[Capitulo 09 - Manejo de errores]];
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
- [[Capitulo 07 - Paquetes, crates y modulos]]
- [[Capitulo 08 - Colecciones comunes]]
- [[Capitulo 09 - Manejo de errores]]
- capitulo 10 para traits y lifetimes

## Siguiente base tecnica inmediata

- Guia objetivo: `https://kylemayes.github.io/vulkanalia/setup/base_code.html`
- Esa base ya presupone comodidad con `Result`, `?`, ownership y colecciones.
