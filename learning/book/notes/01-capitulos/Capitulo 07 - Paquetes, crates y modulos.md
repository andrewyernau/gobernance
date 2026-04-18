---
tags:
  - rust
  - rust/book
  - rust/ch07
status: completado
---

# Capitulo 07 - Paquetes, crates y modulos

## Idea central

Pasar de ejemplos pequenos a una API organizada, donde no todo es publico ni todo vive en el mismo archivo.

## Subcapitulos

- 7.1 Paquetes y crates -> [[Concepto - Paquetes y crates]]
- 7.2 Definiendo modulos para controlar el scope y la privacidad -> [[Concepto - Modulos]]
- 7.3 Paths para referirse a un item en el arbol de modulos -> [[Concepto - Paths y use]]
- 7.4 Incluyendo rutas al scope con la palabra clave `use` -> [[Concepto - Paths y use]]
- 7.5 Separando modulos en diferentes archivos -> [[Concepto - Modulos]]

## Codigo del repo

- `learning/book/ch07`

## Ruta recomendada

1. Distingue bien `package`, crate binario y crate de libreria.
2. Aprende el arbol de modulos y las reglas de privacidad antes de mover archivos.
3. Usa `crate::`, `super::` y `use` para navegar el codigo sin perder claridad.
4. Separa en archivos solo cuando el arbol ya tiene sentido conceptual.

## Por que importa

Sin este capitulo, un proyecto real acaba rapido con:

- `main.rs` demasiado grande;
- APIs internas expuestas por accidente;
- dependencias entre modulos poco claras;
- refactors dolorosos cuando el codigo empieza a crecer.

## Error tipico

- confundir `package`, crate y modulo como si fueran lo mismo;
- marcar todo `pub` para que compile;
- pensar que `use` mueve codigo en vez de crear un atajo de scope.

## Enlaces

- [[Capitulo 06 - Enums y match]]
- [[Concepto - Cargo]]
- [[Concepto - Paquetes y crates]]
- [[Concepto - Modulos]]
- [[Concepto - Paths y use]]
- [[Proyecto - Sandbox con Vulkan]]
