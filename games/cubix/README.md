# Sandbox Game

Bootstrap inicial del proyecto de juego estilo sandbox.

## Objetivo

Separar el proyecto de juego del material de aprendizaje para que ambos puedan crecer sin mezclarse.

## Enfoque recomendado

1. Mundo en memoria antes de pensar en render.
2. Tipos de datos claros para bloques, coordenadas y chunks.
3. Bucle de juego simple.
4. Render y entrada como capa posterior.

## Relación con el aprendizaje

El juego se beneficiará especialmente de:

- `ch04`: ownership y borrowing para pasar datos sin peleas con el compilador;
- `ch05`: structs para modelar bloques, chunks y estado del jugador;
- `ch06`: enums y `match` para tipos de bloque y eventos.

## Estado actual

El crate solo deja un punto de entrada estable y una hoja de ruta mínima para crecer sin contaminar `learning/`.

