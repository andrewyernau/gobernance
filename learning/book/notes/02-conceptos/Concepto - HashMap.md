---
tags:
  - rust
  - rust/concepto
  - rust/hashmap
status: activo
---

# Concepto - HashMap

## Que es

Una coleccion que mapea claves `K` a valores `V` usando hashing para hacer busquedas eficientes por clave.

## Para que sirve

Modela tablas de consulta, caches, indices, contadores y configuraciones donde preguntar por una clave es la operacion principal.

## Por que importa

En un juego acabas necesitando mapas para chunks cargados, catalogos de bloques, inventarios e indices auxiliares.

## Como usarlo

Inserta con `insert`, consulta con `get` y actualiza sin duplicar logica con `entry().or_insert(...)`.

## Cuando usarlo

Cuando una posicion numerica ya no describe bien el problema y necesitas acceder por nombre, coordenada o identificador.

## Error tipico

Olvidar que no garantiza orden estable y no revisar que algunas inserciones mueven valores con ownership.

## Enlaces

- [[Capitulo 08 - Colecciones comunes]]
- [[Concepto - Ownership]]
- [[Concepto - Vec]]
- [[Proyecto - Sandbox con Vulkan]]
