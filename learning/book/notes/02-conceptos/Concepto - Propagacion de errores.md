---
tags:
  - rust
  - rust/concepto
  - rust/errores
status: activo
---

# Concepto - Propagacion de errores

## Que es

La practica de devolver el error al caller en lugar de resolverlo inmediatamente, normalmente con `Result` y el operador `?`.

## Para que sirve

Mantiene limpio el flujo principal y deja la decision de recuperarse, enriquecer contexto o abortar al nivel correcto.

## Por que importa

En programas reales, no todas las funciones deben decidir que hacer ante un fallo. Muchas solo deben comunicarlo bien.

## Como usarlo

Haz que la funcion devuelva `Result<_, _>` y usa `?` para devolver temprano el error si una operacion falla.

## Cuando usarlo

Cuando tu funcion no tiene suficiente contexto para decidir si reintentar, registrar, mostrar al usuario o terminar.

## Error tipico

Intentar usar `?` en funciones que devuelven `()` o propagar errores sin aportar nada de contexto cuando el borde de la app si lo necesita.

## Enlaces

- [[Capitulo 09 - Manejo de errores]]
- [[Concepto - Result]]
- [[Concepto - Option]]
- [[Proyecto - Sandbox con Vulkan]]
