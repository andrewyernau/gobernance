---
tags:
  - rust
  - rust/concepto
  - rust/metodos
status: activo
---

# Concepto - Metodos y funciones asociadas

## Que es

Funciones definidas en un bloque `impl` para un tipo.

## Para que sirve

Permiten expresar comportamiento cerca del dato al que pertenece.

## Diferencia clave

- metodo: recibe `self`, `&self` o `&mut self`
- funcion asociada: no recibe `self`, como `Rectangle::square(...)`

## Por que importa

Ayuda a encapsular reglas del dominio dentro del tipo correcto.

## Cuando usarlo

Cuando una operacion tiene sentido natural sobre una instancia concreta del tipo.

## Error tipico

Crear funciones libres para todo y terminar separando demasiado el comportamiento del dato.

## Enlaces

- [[Capitulo 05 - Structs]]
- [[Concepto - Structs]]
