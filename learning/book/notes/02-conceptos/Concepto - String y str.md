---
tags:
  - rust
  - rust/concepto
  - rust/string
status: activo
---

# Concepto - String y str

## Que es

`String` es texto UTF-8 poseido y mutable; `&str` es una vista prestada sobre texto UTF-8.

## Para que sirve

Te permite decidir si necesitas propiedad del texto o solo leerlo sin copiar.

## Por que importa

Los strings parecen simples hasta que entran ownership, slicing y UTF-8. Rust te obliga a no esconder esa complejidad.

## Como usarlo

Usa `String` cuando el dato debe crecer, cambiar o sobrevivir mas alla del scope actual; usa `&str` para parametros y lecturas temporales.

## Cuando usarlo

Siempre que manejes nombres, comandos, rutas, identificadores o texto de usuario.

## Error tipico

Intentar indexar un `String` por posicion como si cada caracter ocupara un byte o confundir `String` con `&str`.

## Enlaces

- [[Capitulo 08 - Colecciones comunes]]
- [[Concepto - Ownership]]
- [[Concepto - Slices]]
- [[Proyecto - Sandbox con Vulkan]]
