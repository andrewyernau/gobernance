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

## Nota en Obsidian

- `../../learning/book/notes/00-mapas/Proyecto - Sandbox con Vulkan.md`

## Estado actual

El crate solo deja un punto de entrada estable y una hoja de ruta mínima para crecer sin contaminar `learning/`.

## Cosas a trabajar sobre el proyecto
Claro, esto es muy bare metal y 80's. En la realidad puedo simplificar mucho estos procesos con recursos ya MUY OPTIMIZADOS y con mejores manejos que mi cabeza.
### 2D Shape
Rellenar una figura en la pantalla. Dadas 4 coordenadas, con la "y = mx + b" para las 2 primeras lineas, conectamos la "x" izquierda de cada "y" a la "x" derecha de cada "y" y lo mismo para las otras 2 lineas
### 3D - 2D Shape
Rotación y proyección. Dado un "player" y un "objeto" situado en un circulo alrededor del "jugador" (en sus 3 coordenadas pero supongamos "X" para la vertical p.e e "Y" para la horizontal).
Cuando el jugador mira a la izquierda o a la derecha, el objeto se mueve alrededor siguiendo ese círculo, si mira arriba o abajo pasaría lo mismo pero en vertical.
El resúmen matemático sería:
r = x\_1/sin(theta\_1), x2 = r\*sin(theta\_1+theta) = r\*sin(theta)\*cos(theta\_1)+ r\*cos(theta)\*sin(theta\_1) <- ESTO ES LO IMPORTATE
x\_2 = (x\_1\*sin(theta)\*cos(theta\_1))/sin(theta\_1) + (x\_1\*cos(theta)\*sin(theta\_1))/sin(theta\_1) -> x\_2 = z\_1\*sin(theta)+x\_1\*cos(theta)
z\_2 = r\*cos(theta\_1+theta) = (x\_1)/(sin(theta\_1))\*(cos(theta)\*cos(theta\_1)-sin(theta)\*sin(theta\_1)) -> (cos(theta\_1))/(sin(theta\_1)) = (z\_1)/(x\_1)
z\_2 =z\_1\*cos(theta)- x\_1\*sin(theta)

En resumen, intentamos reducir de x,y,z -> x,y usando la proyección (algo más cercano es más "grande"), en resumen es dividir x,y respecto a z
### First cube
Usamos unas matemáticas para generar cuatro coordenadas de cada de las seis caras del cubo. Luego, buscamos cual cara de cada par de las caras paralelas del cubo son las más cercanas al jugador y sólamente pintar esas caras cercanas. Eso se le llama "back face coloring", ya que como mucho, es posible ver en un mismo instante 3 caras de las 6 caras existentes.
Posteriormente subdividir esas caras del cubo que se ven en cubos más pequeños para para poder así pintarlo basado en un mapa de texturas luego rotamos y proyectamos cada esquina de cada usando otras matemáticas y posteriormente usar una función de rellenar la figura para pintar todo (OJO! Esto es diferente si se usan imágenes [habría que igualar el tamaño de los pixeles al tamaño del .png p.e 32x32,16x16...])
### Collisions
Para cada eje, p.e "x", vemos si la x\_1 izquierda o la x\_2 derecha del jugador se solapan con la x\_1 o x\_2 del cubo, asi para cada eje, si coincide en al menos una, existe la colision.
basicamente, cuando el jugador se intenta mover, añadimos el movimiento del jugador y verificamos si hay colisión (si la hay, deshacer el movimiento) [para mejorarlo, mejor no actualizar el movimiento del jugador hasta que no se haya verificado]
### Chunks (DEMO!)
Para hacer el mundo infinito, hay que dividir el mundo en un número de chunks. Cuando un jugador cruza un borde de un chunk, movemos los elementos del array de chunks detrás del jugador ahora delante del jugador. Teletransportar a un jugador a un mismo sitio acaba siendo aburrido, hay que aplicar noise, etc... haciendo el mundo diferente [AQUÍ MI ENFOQUE CAMBIA, SI QUIERO NOISE, PERO LA GENERACIÓN DE MUNDO TRATA DE SER MÁS AMBICIOSA -> Aplicar un perlin noise grande para crear [CONTINENTE,OCÉANO], y a posteriori decorar, transiciones, tipo de continente, etc...]

para el perlin usariamos una funcion donde la x,z de un cubo es el input y el output sería el valor. colocaríamos "cesped" en esa "y", y hasta la "y" más baja (y colocar la "bedrock"), la rellenaríamos simplemente con tierra [esto no acabaría siendo válido si se desea un sistema más sofisticado de cuevas, estructuras...]
Importante que cada vez que colocamos un bloque, estamos comparando también su valor "y" al valor "y" del perln noise output de izquierda a derecha, delante y atrás de esa posición. así sabemos si hay un bloque ahí y entonces el cubo tiene una cara vecina, guardamos eso en el cubo para que el renderer sepa que no hay que pintar caras extra

Crear árboles, estructuras implicaría almacenar en memoria una posición (segun este planteamiento) ya que si nos alejamos y volvemos deberiamos de poder garantizar que siga exactamente ahi (ademas de lo que se construya/se destruya)
### Construction/Destruction
La dificultad en sí residiría en saber cuál es el cubo el cual el usuario trata de interactuar (mirar). Gestionado por el renderer. Verificar con ray casting, puede ser buena idea. Pero la idea principal es dibuaj los alrededores del centro de la pantalla, puede ser un cubo lo que miramos, pero verificar las distancias y marcar unicamente el más cercano (Comprobar además cual es la cara del cubo). Recordar actualizar los valores de cada uno de los bloques vecinos (RENDERING INNECESARIO o RECUPERAR RENDER DE UN BLOQUE QUE PREVIAMENTE ESTABA OCULTO)
### Day/Night
la idea es oscurecer el cielo y la iluminación global
### Transparency
Aquí ejemplos como el agua, cristal... la idea sería para cada valor "y" de cada cuadrado simplemente rellenamos otro valor "y". Esto requiere de mejora, pero es un "hack".
