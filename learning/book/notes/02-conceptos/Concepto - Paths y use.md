---
tags:
  - rust
  - rust/concepto
  - rust/use
status: activo
---

# Concepto - Paths y use

## Que es

Las rutas son la forma de referirte a un item dentro del arbol de modulos. `use` te deja traer esa ruta al scope actual con un nombre mas corto.

## Para que sirve

Permite navegar el codigo con claridad sin repetir rutas completas todo el tiempo.

## Atajos mentales

- `crate::` empieza en la raiz del crate actual.
- `super::` sube al modulo padre.
- `use` crea un atajo de scope, no mueve ni copia codigo.
- `pub use` reexporta y redefine la superficie publica.

## Por que importa

Cuando un proyecto crece, la legibilidad depende mucho de que las rutas sean coherentes y de que la API publica no revele detalles internos por accidente.

## Cuando usarlo

Siempre que una ruta completa se repita o cuando quieras exponer una API mas comoda que la estructura interna real.

## Error tipico

Abusar de `use` hasta esconder de donde sale cada cosa, o usar glob imports sin necesidad.

## Enlaces

- [[Capitulo 07 - Paquetes, crates y modulos]]
- [[Concepto - Modulos]]
- [[Proyecto - Sandbox con Vulkan]]
