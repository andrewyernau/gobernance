---
tags:
  - rust
  - rust/concepto
  - rust/structs
status: activo
---

# Concepto - Structs

## Que es

Un tipo propio que agrupa varios campos relacionados bajo un mismo nombre.

## Para que sirve

Da significado a los datos. Un `struct` no es solo una agrupacion: es una declaracion de modelo.

## Por que importa

Cuando una tupla empieza a necesitar explicaciones, normalmente ya quieres un `struct`.

## Como usarlo

```rust
struct Player {
    name: String,
    health: u32,
}
```

## Cuando usarlo

Cuando varios valores forman una sola entidad del dominio.

## Error tipico

Crear `struct` sin una responsabilidad clara o, al contrario, seguir usando tuplas demasiado tiempo.

## Enlaces

- [[Capitulo 05 - Structs]]
- [[Proyecto - Sandbox con Vulkan]]
- [[Concepto - Metodos y funciones asociadas]]
