---
tags:
  - rust
  - rust/concepto
  - rust/crates
status: activo
---

# Concepto - Paquetes y crates

## Que es

Un `package` es la unidad que maneja Cargo. Un crate es la unidad de compilacion que Rust construye.

## Para que sirve

Te permite entender como se organiza un proyecto: que define Cargo, que compila Rust y donde empieza cada arbol de codigo.

## Regla base

Un package puede contener varios crates binarios, pero como mucho un crate de libreria.

## Por que importa

Si confundes package con crate, luego cuesta entender `src/main.rs`, `src/lib.rs`, dependencias y puntos de entrada.

## Cuando pensarlo

Cada vez que abras un `Cargo.toml` y quieras saber si estas mirando un binario, una libreria o ambos.

## Error tipico

Llamar "crate" a cualquier carpeta y perder la distincion entre organizacion de Cargo y organizacion de modulos.

## Enlaces

- [[Capitulo 07 - Paquetes, crates y modulos]]
- [[Concepto - Cargo]]
- [[Concepto - Modulos]]
