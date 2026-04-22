# Cubix

Proyecto de juego estilo voxel en Rust con renderer Vulkan.

## Ejecutar el proyecto

El comando de PowerShell que se usaba antes sigue siendo valido:

```powershell
PS D:\CODE\rust\gobernance\games\cubix> $env:RUST_LOG="debug"; cargo run --bin cubix
```

Sigue funcionando porque el binario `cubix` sigue existiendo en `src/main.rs`. La diferencia es que ahora `main.rs` solo delega en `cubix::app::run()`, pero el punto de entrada del binario no ha cambiado.

Si no se quieren logs de debug, tambien se puede ejecutar solo:

```powershell
cargo run --bin cubix
```

## Estructura actual

```text
cubix/
  assets/
    fonts/
    models/
    textures/
    ui/
  shaders/
  src/
    app/
    game/
    math/
    renderer/
      vulkan/
    world/
    lib.rs
    main.rs
```

## Patron para futuras implementaciones

La regla principal es esta:

- `app/` coordina la aplicacion.
- `renderer/` dibuja.
- `game/` define estado y logica del jugador o entidades.
- `world/` define bloques, chunks, coordenadas y generacion.
- `math/` contiene tipos o utilidades matematicas reutilizables.
- `assets/` guarda recursos externos como texturas, fuentes o modelos.
- `shaders/` guarda GLSL y SPIR-V del renderer.

El renderer no debe decidir la logica del mundo.  
El mundo no debe depender de Vulkan para existir.

## Donde poner cada `.rs`

### `src/main.rs`

Debe mantenerse pequeno.

Solo debe arrancar el binario y delegar en `app`.

Ejemplo actual:

```rust
fn main() -> anyhow::Result<()> {
    cubix::app::run()
}
```

### `src/app/`

Aqui va la orquestacion de alto nivel de la aplicacion.

Usar esta carpeta para:

- crear ventana;
- inicializar renderer;
- inicializar `GameState`;
- conectar input, update y render loop;
- coordinar subsistemas sin meter detalles internos de cada uno.

Si en el futuro aparece un `App` propio del juego, deberia vivir aqui.

### `src/renderer/`

Aqui va todo lo que sea dibujar o preparar datos para dibujar.

Usar `src/renderer/` para:

- renderer generico;
- API publica del renderer;
- codigo comun de render.

Usar `src/renderer/vulkan/` para:

- bootstrap Vulkan;
- instance, device, swapchain, pipeline, sync;
- command buffers;
- recreacion de swapchain;
- recursos GPU.

Si en el futuro se crean modulos como estos, deberian vivir aqui:

- `camera.rs`
- `mesh.rs`
- `texture.rs`
- `material.rs`
- `chunk_renderer.rs`
- `ui_renderer.rs`

Regla practica:

- si algo habla con `vulkanalia`, casi seguro va en `renderer/vulkan/`;
- si algo convierte mundo en geometria visible, normalmente va en `renderer/` o en el borde entre `world/meshing.rs` y `renderer/`.

### `src/game/`

Aqui va el estado jugable y la logica que no pertenece al mundo estatico.

Usar esta carpeta para:

- `player.rs`
- `entity.rs`
- `input.rs`
- `physics.rs`
- `state.rs`
- futura logica de interaccion, inventario o camara de jugador

Este modulo responde a preguntas como:

- que es un jugador;
- como se mueve;
- que input tiene activo;
- que entidades existen;
- que estado de partida esta cargado.

Si se implementa `Mob`, `Inventory`, `Hotbar`, `RaycastHit` o `InteractionSystem`, este es un sitio razonable para esos `.rs`.

### `src/world/`

Aqui va el modelo del mundo.

Usar esta carpeta para:

- `block.rs`
- `chunk.rs`
- `coords.rs`
- `world.rs`
- `generation.rs`
- `meshing.rs`

Este modulo responde a preguntas como:

- que tipos de bloque existen;
- como se almacena un chunk;
- como se indexa una posicion;
- como se cargan o generan chunks;
- como se obtiene una malla visible a partir de bloques.

Regla importante:

- `Chunk` debe guardar datos del mundo;
- la malla de un chunk debe ser derivada, no la fuente de verdad.

### `src/math/`

Aqui van tipos pequenos y reutilizables que ayudan a evitar estados invalidos o repeticion de logica matematica.

Usar esta carpeta para:

- tipos como `UnitF32`;
- angulos;
- wrappers numericos;
- helpers geometricos pequeños;
- conversiones compartidas.

Si una utilidad es puramente matematica y no pertenece de forma clara a `game/` ni a `world/`, este es su sitio.

## Como anadir un archivo nuevo sin romper el patron

### Si es una nueva pieza del dominio

Ejemplos:

- `block_face.rs`
- `biome.rs`
- `inventory.rs`
- `mob.rs`

Entonces:

1. crear el fichero en la carpeta correcta;
2. declararlo en el `mod.rs` del modulo;
3. si conviene, reexportarlo desde ese `mod.rs`;
4. solo reexportarlo tambien en `src/lib.rs` si debe formar parte de la API raiz del crate.

### Si es una nueva pieza del renderer

Ejemplos:

- `texture.rs`
- `buffer.rs`
- `descriptor.rs`

Entonces:

1. crear el fichero en `src/renderer/` o `src/renderer/vulkan/`;
2. declararlo en el `mod.rs` correspondiente;
3. exponer solo lo necesario.

La idea no es hacer publico todo por defecto.

## Regla de crecimiento

Antes de crear un archivo nuevo, conviene preguntarse:

1. Esto pertenece a aplicacion, renderer, gameplay, mundo o matematicas?
2. Esto es dato fuente o dato derivado?
3. Esto depende de Vulkan de verdad, o solo de logica del juego?

Si una pieza puede existir aunque mañana cambie el backend grafico, no deberia ir en `renderer/`.

## Recursos

`assets/` ya esta preparado para crecer:

- `assets/textures/` para texturas de bloques, atlas y UI
- `assets/ui/` para recursos de interfaz
- `assets/models/` para modelos externos si mas adelante hacen falta
- `assets/fonts/` para fuentes

`shaders/` sigue siendo el sitio correcto para shaders.

## Hoja de ruta e ideas tecnicas

Esta seccion conserva ideas de implementacion pensadas para el proyecto. No es una especificacion cerrada, sino un mapa mental de trabajo. Algunas ideas se han reescrito ligeramente para alinearlas con la arquitectura actual del crate.

### Nota general

El proyecto parte de una base muy bare metal. Eso no es necesariamente malo: ayuda a entender bien las piezas. Con el tiempo podran simplificarse procesos usando recursos mas optimizados, mejores estructuras y separaciones mas limpias entre mundo, gameplay y renderer.

### 2D Shape

Idea base:

- rellenar una figura en pantalla a partir de sus bordes;
- dadas varias coordenadas, recorrer filas y unir el tramo visible entre izquierda y derecha.

Esto es util como intuicion geometrica, pero en el proyecto actual con Vulkan no conviene convertirlo en el centro de la arquitectura. Sirve mas como fundamento mental que como ruta principal de implementacion.

### 3D -> 2D Shape

Idea base:

- partir de coordenadas `x, y, z`;
- aplicar rotacion respecto a la camara o al jugador;
- proyectar a `x, y` de pantalla;
- lo mas cercano se percibe mas grande.

La intuicion matematica de rotacion y proyeccion sigue siendo correcta y valiosa. Aun asi, al crecer el proyecto conviene apoyarse en matrices `model/view/projection` y no en formulas aisladas repartidas por el codigo. Esa parte deberia acabar concentrada entre `math/`, `game/` y mas adelante el pipeline del renderer.

### First cube

Idea base:

- generar las caras de un cubo;
- determinar que caras pueden verse;
- proyectarlas y dibujarlas;
- despues asociarlas a texturas.

Correccion practica para el diseño actual:

- no conviene pensar el cubo solo como una figura a pintar manualmente;
- conviene pensar cada bloque como dato de mundo y la geometria visible como resultado derivado.

En otras palabras:

- `world/` define bloques y vecinos;
- `world/meshing.rs` decide que caras son visibles;
- `renderer/` consume esa malla y la dibuja.

Eso escala mejor que mezclar visibilidad, textura y dibujo en un mismo sitio.

### Collisions

Idea base:

- comprobar solape por ejes entre jugador y bloques;
- si el movimiento produce colision, corregirlo o impedirlo.

Correccion de arquitectura:

- la colision no deberia vivir en el renderer;
- deberia vivir en `game/physics.rs` usando datos de `world/`.

El enfoque sano es:

1. calcular el movimiento deseado;
2. consultar bloques solidos cercanos;
3. resolver colisiones por ejes o con `AABB`;
4. solo entonces actualizar la posicion final del jugador.

Eso encaja bien con el `Player`, `Aabb`, `Transform` y `World` que ya estan preparados.

### Chunks

Idea original:

- dividir el mundo en chunks;
- cuando el jugador cruza limites, reciclar estructuras para simular mundo infinito;
- aplicar noise para que el mundo no sea repetitivo.

Correccion importante para el diseño actual:

- no conviene que la verdad del mundo sea "mover un array de chunks delante y detras";
- conviene que la verdad del mundo sea un mapa por coordenadas, por ejemplo `HashMap<ChunkCoord, Chunk>`.

Luego, encima de eso, si se quiere optimizar, ya se podra:

- cargar chunks alrededor del jugador;
- descargar chunks lejanos;
- reciclar buffers o mallas;
- cachear generacion.

La idea del noise sigue siendo valida. De hecho, la direccion ambiciosa de usar capas grandes para continente/oceano y luego decorar por regiones tiene mucho mas sentido que una simple altura plana repetida.

Ruta razonable:

- una capa de forma global: continente, oceano, macro relieve;
- una capa regional: bioma, transiciones, temperatura, humedad;
- una capa local: terreno, cuevas, decoracion, estructuras;
- una capa persistente: bloques colocados/rotos por el jugador.

La generacion procedural deberia vivir primero en `world/generation.rs`.

### Vecinos visibles y caras ocultas

Idea base:

- si un bloque tiene vecino solido en una cara, esa cara no se dibuja.

Eso sigue siendo totalmente correcto, pero la informacion de "caras visibles" no deberia almacenarse como la verdad principal de cada bloque salvo que un dia haya una necesidad de optimizacion muy concreta.

La regla recomendada es:

- el bloque guarda su tipo;
- la visibilidad se calcula al generar o regenerar la malla del chunk.

Eso encaja directamente con `world/meshing.rs`.

### Arboles, estructuras y persistencia

Idea base:

- si el jugador se aleja y vuelve, arboles, estructuras y modificaciones deben seguir ahi.

Eso implica distinguir entre:

- generacion base reproducible por semilla;
- cambios persistentes aplicados encima.

Arquitectonicamente, eso apunta a dos capas:

- mundo base generado proceduralmente;
- delta persistente del mundo: construcciones, destrucciones, estructuras fijadas, etc.

### Construction / Destruction

Idea base:

- detectar que bloque esta mirando el jugador;
- elegir el mas cercano;
- saber en que cara interactua;
- actualizar bloques vecinos si cambia la visibilidad.

Correccion importante:

- esto no deberia estar "gestionado por el renderer" como logica principal;
- deberia vivir en un sistema de interaccion dentro de `game/`, usando datos de camara/jugador y consultas al `world/`.

El renderer puede ayudar de forma indirecta:

- exponiendo la camara usada para dibujar;
- respetando el centro de pantalla;
- mostrando resaltado visual del bloque objetivo.

Pero la decision de que bloque se rompe o se coloca pertenece a gameplay/interaccion, no al backend grafico.

Una ruta sana seria:

1. calcular un rayo desde la camara del jugador;
2. intersectarlo con bloques del mundo;
3. obtener `BlockCoord` y cara impactada;
4. modificar el mundo;
5. marcar chunk propio y chunks vecinos como sucios para remallar.

### Day / Night

Idea base:

- oscurecer cielo e iluminacion global.

Eso es una buena primera aproximacion. Mas adelante puede separarse en:

- color de cielo;
- luz ambiental global;
- direccion e intensidad de una luz principal;
- niebla o atmosfera si hiciera falta.

Eso acabara tocando tanto `game/` como `renderer/`.

### Transparency

Idea base:

- casos como agua o cristal necesitan un tratamiento distinto.

La intuicion de que requieren un camino especial es correcta. El "hack" de rellenar diferente puede servir para experimentos tempranos, pero a medio plazo conviene separar:

- bloques opacos;
- bloques transparentes;
- orden de dibujo o pipeline especifico;
- reglas de mezcla y profundidad.

Eso deberia acabar reflejado en el renderer y posiblemente en el meshing de chunks.

## Estado actual

Ahora mismo el proyecto ya tiene:

- una app de arranque;
- un renderer Vulkan modularizado;
- una base de `game/`;
- una base de `world/`;
- una base de `math/`;
- carpetas de assets listas para ampliarse.

El siguiente crecimiento sano seria conectar `GameState` con `app`, y despues empezar a bajar mundo real a geometria real.
