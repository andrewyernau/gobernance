---
tags:
  - rust
  - rust/concepto
  - rust/modulos
status: activo
---

# Concepto - Modulos

## Que es

El mecanismo de Rust para agrupar codigo y decidir que queda privado o publico.

## Para que sirve

Sirve para construir un arbol de codigo mantenible en lugar de dejar todo en un archivo o todo expuesto.

## Por que importa

Cuando el proyecto crece, los modulos dejan de ser decoracion: pasan a definir la forma real de tu API.

## Reglas utiles

- `mod` declara un modulo dentro del arbol.
- `pub` expone un item hacia fuera.
- por defecto, todo es privado.

## Cuando usarlo

En cuanto empiezas a tener tipos, funciones o responsabilidades que ya no caben comodamente en un solo archivo mental.

## Error tipico

Marcar todo como `pub` en lugar de decidir con cuidado que forma parte de la API.

## Enlaces

- [[Capitulo 07 - Paquetes, crates y modulos]]
- [[Concepto - Paths y use]]
- [[Proyecto - Sandbox con Vulkan]]
