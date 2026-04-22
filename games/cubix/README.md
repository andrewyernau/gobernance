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

## Estado actual

Ahora mismo el proyecto ya tiene:

- una app de arranque;
- un renderer Vulkan modularizado;
- una base de `game/`;
- una base de `world/`;
- una base de `math/`;
- carpetas de assets listas para ampliarse.

El siguiente crecimiento sano seria conectar `GameState` con `app`, y despues empezar a bajar mundo real a geometria real.
