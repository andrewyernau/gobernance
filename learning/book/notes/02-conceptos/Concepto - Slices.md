---
tags:
  - rust
  - rust/concepto
  - rust/slices
status: activo
---

# Concepto - Slices

## Que es

Una vista sobre una parte de una coleccion, sin copiarla.

## Para que sirve

Permite trabajar con fragmentos de `String`, `str`, arrays o vectores sin mover los datos.

## Por que importa

Es una de las primeras formas de notar que Rust prefiere referencias expresivas a copias silenciosas.

## Como reconocerlo

- `&str`
- `&text[..index]`
- `&values[1..4]`

## Cuando usarlo

Cuando una funcion solo necesita leer una porcion de datos ya existente.

## Error tipico

Devolver indices cuando en realidad puedes devolver un slice con mas contexto y menos riesgo.

## Enlaces

- [[Capitulo 04 - Ownership]]
- [[Concepto - Borrowing y referencias]]
