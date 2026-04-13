---
tags:
  - rust
  - rust/concepto
  - rust/enums
status: activo
---

# Concepto - Enums

## Que es

Un tipo que puede tomar una de varias variantes posibles.

## Para que sirve

Modela estados cerrados y hace imposible representar combinaciones invalidas si disenas bien el tipo.

## Por que importa

En Rust, muchos problemas que en otros lenguajes se resuelven con strings o banderas dispersas, aqui se modelan mejor con `enum`.

## Como usarlo

```rust
enum BlockType {
    Dirt,
    Stone,
    Water,
}
```

## Cuando usarlo

Cuando el conjunto de posibilidades es conocido y finito.

## Error tipico

Usar `String` para representar estados cerrados y luego comparar texto por todas partes.

## Enlaces

- [[Capitulo 06 - Enums y match]]
- [[Concepto - Match]]
- [[Proyecto - Sandbox con Vulkan]]
