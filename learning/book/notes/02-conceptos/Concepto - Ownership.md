---
tags:
  - rust
  - rust/concepto
  - rust/ownership
status: activo
---

# Concepto - Ownership

## Que es

El sistema por el que Rust sabe quien es responsable de cada dato y cuando debe liberarlo.

## Para que sirve

Evita dobles liberaciones, uso de memoria invalida y muchos bugs de concurrencia.

## Las tres reglas base

1. Cada valor tiene un propietario.
2. Solo puede haber un propietario a la vez.
3. Cuando el propietario sale de scope, el valor se libera.

## Por que existe

Rust quiere seguridad de memoria sin garbage collector. Ownership es el precio y tambien la ventaja.

## Como reconocer un movimiento

Con tipos como `String`, hacer `let b = a;` mueve el valor a `b`.

## Cuando pensar activamente en esto

Siempre que pases `String`, `Vec`, `struct` o cualquier dato que posea memoria.

## Error tipico

Responder a todo con `clone()`. A veces hace falta, pero usarlo sin criterio es esconder el problema.

## Enlaces

- [[Capitulo 04 - Ownership]]
- [[Concepto - Borrowing y referencias]]
- [[Proyecto - Sandbox con Vulkan]]
