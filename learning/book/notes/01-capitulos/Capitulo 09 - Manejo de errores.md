---
tags:
  - rust
  - rust/book
  - rust/ch09
status: completado
---

# Capitulo 09 - Manejo de errores

## Idea central

Distinguir entre fallos recuperables y errores de programacion, y expresar esa diferencia en el tipo y en el flujo del programa.

## Subcapitulos

- 9.1 Errores irrecuperables con `panic!` -> [[Concepto - Panic]]
- 9.2 Errores recuperables con `Result` -> [[Concepto - Result]]
- 9.3 Propagacion y criterio para fallar o recuperarse -> [[Concepto - Propagacion de errores]]

## Codigo del repo

- `learning/book/ch09`

## Ruta recomendada

1. Usa `panic!` para invariantes rotas o estados imposibles, no para input externo normal.
2. Usa `Result` para parseo, IO, red, assets, configuracion y cualquier operacion que pueda fallar en produccion.
3. Aprende a propagar con `?` para no enterrar el flujo entre `match` repetitivos.
4. Decide en los bordes de la app donde registras, enriqueces contexto o detienes el proceso.

## Por que importa

En cuanto empieces con `vulkanalia`, apareceran funciones que devuelven `Result` por todas partes:

- creacion de ventana y event loop;
- inicializacion de Vulkan y recursos del driver;
- carga de shaders, assets y configuracion;
- validacion de invariantes internas del motor.

Si no distingues bien entre `panic!` y `Result`, acabaras con un prototipo fragil o con errores tragados sin contexto.

## Error tipico

- abusar de `unwrap()` y `expect()` en rutas de produccion;
- convertir cualquier fallo externo en `panic!`;
- propagar errores sin contexto cuando el borde de la app si necesita explicarlos;
- no reservar `panic!` para bugs o estados imposibles.

## Enlaces

- [[Capitulo 08 - Colecciones comunes]]
- [[Concepto - Panic]]
- [[Concepto - Result]]
- [[Concepto - Propagacion de errores]]
- [[Concepto - Match]]
- [[Proyecto - Sandbox con Vulkan]]
