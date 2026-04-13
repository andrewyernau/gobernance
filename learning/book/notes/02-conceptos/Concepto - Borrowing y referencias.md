---
tags:
  - rust
  - rust/concepto
  - rust/borrowing
status: activo
---

# Concepto - Borrowing y referencias

## Que es

Pedir acceso a un dato sin convertirte en su propietario.

## Para que sirve

Permite reutilizar datos sin moverlos y sin clonar de forma innecesaria.

## Regla mental util

- `&T` presta lectura
- `&mut T` presta escritura

## Por que existe

Ownership por si solo seria demasiado rigido. Borrowing da flexibilidad sin perder seguridad.

## Regla importante

No puedes tener una referencia mutable y otra referencia activa al mismo dato al mismo tiempo.

## Cuando usarlo

Cuando una funcion necesita trabajar con un dato, pero no quedarselo.

## Error tipico

Intentar mutar un valor mientras sigue prestado en otra parte.

## Enlaces

- [[Capitulo 04 - Ownership]]
- [[Concepto - Ownership]]
- [[Concepto - Slices]]
