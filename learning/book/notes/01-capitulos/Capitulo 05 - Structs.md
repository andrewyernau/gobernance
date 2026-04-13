---
tags:
  - rust
  - rust/book
  - rust/ch05
status: completado
---

# Capitulo 05 - Structs

## Idea central

Pasar de datos anonimos a tipos con significado.

## Subcapitulos

- 5.1 Definiendo e instanciando structs -> [[Concepto - Structs]]
- 5.2 Programa de ejemplo usando structs -> [[Concepto - Structs]]
- 5.3 Sintaxis de metodos -> [[Concepto - Metodos y funciones asociadas]]

## Codigo del repo

- `learning/book/ch05/defining_structs`
- `learning/book/ch05/example_structs`
- `learning/book/ch05/method_syntax`

## Ruta recomendada

1. Primero define la forma del dato.
2. Luego compara una solucion pobre frente a una con `struct`.
3. Por ultimo mueve el comportamiento al tipo con metodos.

## Por que importa

Sin `struct`, un sandbox se degrada rapido en tuplas sueltas, parametros ambiguos y estado dificil de mantener.

## Pregunta de control

Si mañana modelas un `Chunk`, sabrias decidir que campos guarda y que metodos deberia exponer?

## Enlaces

- [[Capitulo 04 - Ownership]]
- [[Capitulo 06 - Enums y match]]
- [[Concepto - Structs]]
- [[Proyecto - Sandbox con Vulkan]]
