---
tags:
  - rust
  - vulkan
  - cubix
  - renderer
status: activo
---

# Proyecto - Cubix - Triangulo y bootstrap Vulkan

## Que documenta esta nota

Esta nota documenta el estado real alcanzado en `games/cubix` despues de conseguir el primer triangulo en pantalla con Vulkan. No esta pensada como un resumen superficial del tutorial, sino como una guia de mantenimiento:

- para entender por que el renderer funciona;
- para saber que partes son obligatorias y cuales son configurables;
- para ubicar donde tocar cuando quieras avanzar a figuras 3D, texturas, controles, movimiento, chunks o scene rendering real.

La referencia principal ha sido el tutorial de Vulkanalia:

- Overview: `https://kylemayes.github.io/vulkanalia/overview.html`
- Swapchain: `https://kylemayes.github.io/vulkanalia/presentation/swapchain.html`
- Rendering and presentation: `https://kylemayes.github.io/vulkanalia/drawing/rendering_and_presentation.html`
- Swapchain recreation: `https://kylemayes.github.io/vulkanalia/swapchain/recreation.html`

## Estado exacto alcanzado

El estado actual del proyecto no es solo "dibujar un triangulo". Lo que ya existe es una base funcional de renderer Vulkan con estas piezas:

- ventana creada con `winit`;
- `Instance` de Vulkan con validacion en debug;
- `DebugUtilsMessengerEXT` para mensajes de validacion;
- `SurfaceKHR` asociada a la ventana;
- seleccion de `PhysicalDevice`;
- seleccion de colas de graficos y presentacion;
- `Device` logico;
- `SwapchainKHR`;
- imagenes del swapchain y sus `ImageView`;
- `RenderPass`;
- `PipelineLayout` y `GraphicsPipeline`;
- `Framebuffer` por imagen del swapchain;
- `CommandPool`;
- `CommandBuffer` primario por framebuffer;
- semaforos y fences para sincronizacion;
- bucle de render que hace `acquire -> submit -> present`;
- recreacion de swapchain cuando Vulkan devuelve `OUT_OF_DATE_KHR` o `SUBOPTIMAL_KHR`.

Tambien hay varias cosas que todavia no existen:

- no hay `Vertex Buffer`;
- no hay `Index Buffer`;
- no hay `Uniform Buffer`;
- no hay `Descriptor Set Layout`, `Descriptor Pool` ni `Descriptor Sets`;
- no hay texturas ni samplers;
- no hay depth buffer;
- no hay modelos 3D cargados;
- no hay input de juego, solo cierre de ventana;
- no hay `GameState`, `World`, `Player`, `Chunk`, `Camera` ni simulacion;
- no hay separacion en modulos Rust: todo el bootstrap Vulkan vive todavia en `games/cubix/src/main.rs`.

Dato importante para no perder perspectiva: el `main.rs` actual tiene `876` lineas no vacias. Eso significa que ya has superado claramente el punto del "triangulo minimo". La mayor parte del trabajo no ha sido geometria, sino infraestructura.

## Conclusion mental que debes interiorizar

El triangulo visible es solo la prueba de que toda la cadena Vulkan esta bien conectada. Lo importante no es el triangulo en si, sino que ahora ya tienes:

- una ventana que puede presentar imagenes;
- una GPU seleccionada correctamente;
- una tuberia grafica valida;
- buffers de comandos grabados;
- sincronizacion suficiente para no pisarte imagenes en vuelo;
- una base sobre la que puedes empezar a sustituir datos hardcodeados por datos reales del juego.

Dicho de otra forma: el primer hito no es "he dibujado un triangulo", sino "ya tengo un renderer bootstrappeado que sabe hablar con la GPU de forma correcta".

## Mapa mental del programa

Este es el grafo mental mas util para entender el estado actual:

```text
Window (winit)
  -> Instance
    -> Debug Messenger
    -> Surface
    -> Physical Device
      -> QueueFamilyIndices
      -> Logical Device
        -> Graphics Queue
        -> Present Queue
        -> Swapchain
          -> Swapchain Images
            -> Image Views
          -> Render Pass
          -> Pipeline Layout
          -> Graphics Pipeline
          -> Framebuffers
          -> Command Buffers
          -> Render-finished semaphores (por imagen)
          -> Images-in-flight fences (por imagen)
        -> Command Pool
        -> Image-available semaphores (por frame)
        -> In-flight fences (por frame)
```

La regla de oro es esta:

- lo de arriba habilita lo de abajo;
- lo de abajo depende de lo de arriba;
- si algo cambia arriba, muchas veces hay que recrear cosas de abajo;
- por eso la destruccion va en orden inverso al de creacion.

## Ficheros que forman este hito

### `games/cubix/src/main.rs`

Es el centro absoluto del renderer actual. Aqui viven:

- bootstrap de Vulkan;
- seleccion de dispositivo;
- presentacion;
- pipeline;
- command buffers;
- sincronizacion;
- recreacion de swapchain;
- destruccion final.

### `games/cubix/shaders/shader.vert`

Contiene el triangulo hardcodeado. No hay vertices en CPU ni `Vertex Buffer`; el shader usa `gl_VertexIndex` para leer dos arrays:

- `positions[3]`;
- `colors[3]`.

### `games/cubix/shaders/shader.frag`

Recibe el color interpolado desde el vertex shader y lo escribe en `outColor`.

### `games/cubix/shaders/vert.spv` y `games/cubix/shaders/frag.spv`

Son los binarios SPIR-V que realmente carga Vulkan. Cambiar el GLSL sin recompilar estos ficheros no cambia nada en runtime.

### `games/cubix/scripts/compile.bat`

Compila GLSL a SPIR-V con `glslc.exe`. Ahora mismo esta atado a un path concreto del Vulkan SDK en Windows. Eso funciona en tu maquina, pero es una configuracion local, no una verdad universal del proyecto.

### `games/cubix/Cargo.toml`

Marca bien hacia donde va el proyecto:

- `vulkanalia` y `winit` para renderer y ventana;
- `cgmath` preparado para transformaciones futuras;
- `png` preparado para texturas;
- `tobj` preparado para carga de modelos.

Es decir, varias dependencias de etapas futuras ya estan declaradas aunque todavia no esten conectadas al pipeline.

## Orden real de creacion

El orden actual importa. No es decorativo.

1. `main`
   Crea logger, `EventLoop` y `Window`.
2. `App::create`
   Arranca todo el bootstrap Vulkan.
3. `LibloadingLoader` + `Entry`
   Permiten cargar la libreria Vulkan del sistema.
4. `create_instance`
   Crea el `Instance`, activa layers y extensiones, y prepara el debug messenger.
5. `vk_window::create_surface`
   Crea la `SurfaceKHR` para presentar imagenes en la ventana.
6. `pick_physical_device`
   Elige una GPU que soporte colas adecuadas y extensiones necesarias.
7. `create_logical_device`
   Crea el `Device` logico y recupera las colas de graficos y presentacion.
8. `create_swapchain`
   Crea el swapchain y obtiene sus imagenes.
9. `create_swapchain_image_views`
   Envuelve cada imagen del swapchain en una vista utilizable por el pipeline.
10. `create_render_pass`
    Define como va a usarse el attachment de color.
11. `create_pipeline`
    Carga shaders, define estados fijos y crea el pipeline grafico.
12. `create_framebuffers`
    Asocia cada `ImageView` con el `RenderPass`.
13. `create_command_pool`
    Reserva el contexto desde el que se asignaran command buffers.
14. `create_command_buffers`
    Graba el render pass, bind del pipeline y `cmd_draw(3, 1, 0, 0)`.
15. `create_sync_objects`
    Crea semaforos y fences para coordinar CPU, GPU y presentacion.

Si alteras este orden sin entender las dependencias, romperas el renderer.

## Orden real de destruccion

Tambien importa. Vulkan no perdona destruir recursos mientras siguen en uso.

1. `device_wait_idle`
   Primero se espera a que el device termine todo.
2. `destroy_swapchain`
   Libera recursos ligados al swapchain:
   `render_finished_semaphores`, `images_in_flight`, `command_buffers`, `framebuffers`, `pipeline`, `pipeline_layout`, `render_pass`, `swapchain_image_views`, `swapchain`, `swapchain_images`.
3. Recursos no ligados al swapchain
   `in_flight_fences`, `image_available_semaphores`, `command_pool`, `device`.
4. Recursos de instancia
   `debug messenger`, `surface`, `instance`.

Regla practica:

- si algo depende del `Device`, no lo destruyas despues del `Device`;
- si algo depende del `Swapchain`, no lo dejes vivo al recrear o destruir el swapchain;
- si algo puede seguir en vuelo, espera antes.

## Matriz de dependencias

| Pieza | Donde se crea | De que depende | Quien la usa despues | Cuando obliga a tocar algo |
| --- | --- | --- | --- | --- |
| `Instance` | `create_instance` | loader, layers, extensiones | surface, debug, device selection | casi todo depende de el |
| `DebugUtilsMessengerEXT` | `create_instance` | instance + ext debug utils | logs de validacion | solo en debug |
| `SurfaceKHR` | `App::create` | instance + window | queue family checks, swapchain | si cambia ventana/plataforma |
| `PhysicalDevice` | `pick_physical_device` | instance + surface | logical device, swapchain support | si cambian requisitos de GPU |
| `QueueFamilyIndices` | helper | physical device + surface | logical device, command pool, swapchain sharing | si cambian colas necesarias |
| `Device` | `create_logical_device` | physical device + extensions + queues | todo lo demas del renderer | base de todos los recursos de device |
| `graphics_queue` | `create_logical_device` | device | queue submit | si cambias familia de colas |
| `present_queue` | `create_logical_device` | device | queue present | si cambias familia de colas |
| `SwapchainKHR` | `create_swapchain` | device + surface + support details | image views, render loop | si cambia tamano, formato o compatibilidad |
| `swapchain_images` | `create_swapchain` | swapchain | image views, command buffers, sync por imagen | siempre ligadas al swapchain |
| `swapchain_image_views` | `create_swapchain_image_views` | swapchain images + format | framebuffers | si cambia swapchain o format |
| `RenderPass` | `create_render_pass` | swapchain format | pipeline, command buffers, framebuffers | si cambia formato o attachments |
| `PipelineLayout` | `create_pipeline` | descriptor layouts / push constants | pipeline | si anades uniforms o push constants |
| `Pipeline` | `create_pipeline` | shaders + render pass + fixed state + extent | command buffers | si cambian shaders, viewport estatico, vertex input, blend, cull, etc. |
| `Framebuffers` | `create_framebuffers` | image views + render pass + extent | command buffers | si cambia swapchain o render pass |
| `CommandPool` | `create_command_pool` | graphics queue family | command buffers | si cambia estrategia de grabacion/reset |
| `CommandBuffers` | `create_command_buffers` | command pool + framebuffers + render pass + pipeline | `render()` | si cambia geometria, pipeline, framebuffers o estrategia de grabacion |
| `image_available_semaphores` | `create_sync_objects` | device | `render()` antes de submit | ligados a frames en vuelo |
| `in_flight_fences` | `create_sync_objects` | device | CPU espera antes de reutilizar frame | ligados a frames en vuelo |
| `render_finished_semaphores` | `create_swapchain_sync_objects` | device + swapchain images | `render()` antes de present | se recrean con swapchain |
| `images_in_flight` | `create_swapchain_sync_objects` | swapchain images | evita reutilizar una imagen aun ocupada | se recrean con swapchain |

## Partes que son asi si o si

Estas son las verdades estructurales del renderer actual. Si las olvidas, te perderas.

- Sin `SurfaceKHR` no puedes comprobar soporte real de presentacion.
- Sin `VK_KHR_swapchain` no puedes presentar imagenes en pantalla.
- `RenderPass`, `Pipeline` y `Framebuffer` tienen que estar alineados en attachments y formato.
- Los `CommandBuffer` se graban contra un `RenderPass`, un `Framebuffer` y un `Pipeline` concretos.
- `cmd_draw(3, 1, 0, 0)` tiene sentido solo porque el vertex shader produce exactamente tres vertices.
- Si cambias el tamano o compatibilidad del swapchain, debes recrear los objetos que dependen de el.
- Antes de destruir o recrear recursos en uso debes esperar con `device_wait_idle` o con sincronizacion equivalente.
- La destruccion correcta va en orden inverso a las dependencias.

## Partes configurables y donde se tocan

| Tema | Donde se toca | Comentario |
| --- | --- | --- |
| Titulo de ventana | `main` | `.with_title("cubix")` |
| Tamano inicial de ventana | `main` | `.with_inner_size(LogicalSize::new(1024, 768))` |
| Activar validacion | `VALIDATION_ENABLED` | ahora depende de `debug_assertions` |
| Layer de validacion | `VALIDATION_LAYER` | actualmente `VK_LAYER_KHRONOS_validation` |
| Extensiones de instance | `create_instance` | `vk_window::get_required_instance_extensions(window)` mas debug utils en debug |
| Extensiones de device | `DEVICE_EXTENSIONS` y `create_logical_device` | ahora mismo solo swapchain |
| Numero de frames en vuelo | `MAX_FRAMES_IN_FLIGHT` | ahora `2` |
| Formato preferido del swapchain | `get_swapchain_surface_format` | intenta `B8G8R8A8_SRGB` + `SRGB_NONLINEAR` |
| Present mode preferido | `get_swapchain_present_mode` | intenta `MAILBOX`, cae a `FIFO` |
| Extent del swapchain | `get_swapchain_extent` | usa `current_extent` o clampa al tamano de ventana |
| Color de fondo | `create_command_buffers` | `ClearColorValue { float32: [0.0, 0.0, 0.0, 1.0] }` |
| Topologia de primitivas | `create_pipeline` | ahora `TRIANGLE_LIST` |
| Cull mode | `create_pipeline` | ahora `BACK` |
| Front face | `create_pipeline` | ahora `CLOCKWISE` |
| Polygon mode | `create_pipeline` | ahora `FILL` |
| Blending | `create_pipeline` | desactivado |
| Viewport y scissor | `create_pipeline` | definidos de forma estatica con el extent actual |
| Numero de vertices dibujados | `create_command_buffers` | ahora `cmd_draw(..., 3, ...)` |
| Geometria visible | `shader.vert` | ahora hardcodeada en arrays GLSL |
| Color visible | `shader.vert` y `shader.frag` | interpolacion simple RGB |
| Compilacion de shaders | `scripts/compile.bat` | local a Windows y SDK concreto |

## Matices criticos que te pueden ahorrar horas

### 1. El triangulo esta en el shader, no en Rust

Esto es probablemente el detalle mas importante del estado actual.

Ahora mismo:

- `vertex_input_state` esta vacio;
- no se hace `bind_vertex_buffers`;
- no existe struct `Vertex` en Rust;
- el vertex shader usa `gl_VertexIndex` para generar posicion y color.

Eso implica que, si quieres dibujar otra geometria seria, tu siguiente salto natural no es "tocar un poco el shader", sino entrar en vertex buffers.

### 2. `front_face = CLOCKWISE` no es un valor inocente

El pipeline actual tambien tiene:

- `cull_mode = BACK`;
- `front_face = CLOCKWISE`.

Si cambias el orden de los vertices o los indices y una figura desaparece, no siempre sera porque el shader este mal. Puede ser simplemente que ahora estas definiendo la cara frontal al reves y el rasterizador la este descartando.

### 3. El viewport es estatico

Como el viewport y el scissor se crean con `swapchain_extent` dentro del pipeline, al recrear el swapchain tambien recreas el pipeline. Eso es coherente con el estado actual.

Si algun dia haces viewport/scissor dinamicos, podras reducir recreaciones.

### 4. Ya has adelantado parte del capitulo 21

Tu codigo ya tiene:

- `recreate_swapchain`;
- `destroy_swapchain`;
- manejo de `OUT_OF_DATE_KHR`;
- manejo de `SUBOPTIMAL_KHR`;
- proteccion contra tamanos `0 x 0`.

Pero todavia no esta implementado el enfoque completo de "marcar `resized = true` desde `WindowEvent::Resized`". Eso significa:

- ya no estas en el estado puro del capitulo 20;
- pero tampoco has cerrado el 21 de forma total y explicita.

### 5. La sincronizacion mezcla dos granularidades

En este proyecto actual:

- `image_available_semaphores` e `in_flight_fences` son por frame en vuelo;
- `render_finished_semaphores` e `images_in_flight` son por imagen del swapchain.

No es la unica estrategia posible, pero si es la estrategia concreta de este codigo. Manana, si refactorizas sincronizacion, documenta si pasas a semaforos por frame o mantienes este esquema mixto.

### 6. El command pool no esta preparado para regrabado fino

`create_command_pool` usa `vk::CommandPoolCreateFlags::empty()`.

Eso encaja con el estado actual porque:

- grabas command buffers una vez;
- los recreas cuando recreas el swapchain;
- no regrabas contenido dinamico por frame.

Pero cuando quieras chunks dinamicos, entidades o UI, este punto deja de ser neutro. Puede interesarte `RESET_COMMAND_BUFFER` o una estrategia de reset del pool.

## Que hace cada modulo conceptual dentro del `main.rs`

Aunque todo vive en un solo fichero, mentalmente ya puedes separarlo asi.

### Modulo conceptual: plataforma y event loop

Funciones y zonas:

- `main`

Responsabilidad:

- crear la ventana;
- arrancar el event loop;
- pedir `redraw` continuamente;
- llamar a `app.render(&window)` cuando toca;
- destruir la app al cerrar.

Que no hace todavia:

- no procesa teclado;
- no procesa raton;
- no calcula delta time;
- no actualiza simulacion del juego;
- no separa update y render.

Consecuencia:

Tu `main loop` ya sirve como render loop, pero todavia no es un game loop completo.

### Modulo conceptual: bootstrap de Vulkan

Funciones y tipos:

- `create_instance`
- `pick_physical_device`
- `create_logical_device`
- `QueueFamilyIndices`
- `SwapchainSupport`
- `check_physical_device`
- `check_physical_device_extensions`
- `debug_callback`

Responsabilidad:

- conectar con Vulkan;
- activar validacion;
- elegir una GPU valida;
- elegir familias de colas;
- crear el `Device`.

Si manana algo falla aqui, normalmente el problema sera uno de estos:

- falta una extension;
- la GPU no soporta presentacion sobre la `Surface`;
- las validation layers no estan instaladas;
- el Vulkan SDK o el driver no estan bien montados.

### Modulo conceptual: presentacion

Funciones:

- `create_swapchain`
- `create_swapchain_image_views`
- `get_swapchain_surface_format`
- `get_swapchain_present_mode`
- `get_swapchain_extent`

Responsabilidad:

- decidir como se crean las imagenes que acabaran en pantalla;
- escoger formato, present mode y extent;
- exponer cada imagen como `ImageView`.

Esta capa existe porque Vulkan no tiene un framebuffer por defecto.

### Modulo conceptual: pipeline grafico

Funciones:

- `create_render_pass`
- `create_pipeline`
- `create_shader_module`

Responsabilidad:

- declarar el attachment de color;
- definir la tuberia grafica;
- cargar shaders compilados;
- configurar estados fijos.

Esta es la zona que mas vas a tocar al pasar a:

- vertex input real;
- uniforms;
- texturas;
- depth buffering;
- blending;
- push constants.

### Modulo conceptual: grabacion de comandos

Funciones:

- `create_command_pool`
- `create_framebuffers`
- `create_command_buffers`

Responsabilidad:

- reservar command buffers;
- grabar las ordenes que la GPU ejecutara;
- asociar cada command buffer a un framebuffer concreto del swapchain.

Ahora mismo el contenido grabado es minimo:

1. empezar render pass;
2. bind pipeline;
3. dibujar 3 vertices;
4. cerrar render pass.

### Modulo conceptual: sincronizacion

Funciones:

- `create_sync_objects`
- `create_swapchain_sync_objects`
- `render`

Responsabilidad:

- evitar que CPU y GPU se pisen;
- evitar reutilizar una imagen del swapchain antes de tiempo;
- ordenar acquire, draw y present.

Sin esta capa, el programa podria:

- colgarse;
- corromper recursos;
- disparar validacion;
- crecer en memoria;
- dibujar sobre imagenes todavia en uso.

### Modulo conceptual: lifetime management

Funciones:

- `recreate_swapchain`
- `destroy_swapchain`
- `destroy`

Responsabilidad:

- recrear recursos dependientes del swapchain;
- liberar recursos en orden correcto;
- esperar al device antes de destruir.

Esta capa es la que convierte un ejemplo que "a veces pinta algo" en un programa que ya empieza a comportarse como software grafico serio.

## El `main loop` actual, explicado paso a paso

La secuencia real de `render()` es esta:

```text
1. Esperar a que termine el fence del frame actual.
2. Adquirir una imagen del swapchain.
3. Si el swapchain esta obsoleto, recrearlo y salir del frame.
4. Si esa imagen concreta sigue en vuelo, esperar a su fence.
5. Marcar que esa imagen pasa a estar asociada al fence del frame actual.
6. Preparar el submit:
   - espera el semaforo de imagen disponible;
   - ejecuta el command buffer de esa imagen;
   - senaliza el semaforo de render terminado.
7. Resetear el fence del frame actual.
8. Hacer queue submit en la cola de graficos.
9. Presentar esa imagen en la cola de presentacion.
10. Si presentation indica cambio de swapchain, recrearlo.
11. Avanzar al siguiente frame en vuelo.
```

Interpretacion importante:

- la CPU no dibuja el triangulo; la CPU prepara y envia trabajo;
- la GPU ejecuta despues, de forma asincrona;
- por eso fences y semaforos no son adornos, son parte de la correccion del programa.

## Lo que esta hardcodeado hoy

Este inventario es importante porque marca exactamente por donde vas a romper el estado actual cuando empieces a avanzar.

- la figura visible es un triangulo de 3 vertices;
- las posiciones estan hardcodeadas en `shader.vert`;
- los colores estan hardcodeados en `shader.vert`;
- el fondo es negro;
- hay un unico render pass de color;
- no existe profundidad;
- no existe camara;
- no existe transformacion `model/view/projection`;
- no existe geometria en memoria CPU preparada para subirse a GPU;
- no existe asset pipeline real, salvo compilacion manual de shaders.

## Lo que ya puedes empezar a hacer para el juego

Si, ya puedes empezar a construir cosas del juego, pero no todas en la misma capa.

Puedes empezar ya con bastante seguridad en la capa CPU:

- `GameState`;
- `Player`;
- `Camera` conceptual;
- `Block`, `Chunk`, `World`;
- input de teclado y raton;
- logica de movimiento;
- simulacion de chunks;
- visibilidad de caras;
- generacion procedural;
- colisiones.

Lo que todavia no esta listo para crecer de forma natural en GPU es:

- geometria 3D real en buffers;
- texturas;
- matrices de camara;
- depth testing;
- carga de modelos;
- render dinamico de mundo.

Conclusion practica:

- mundo y gameplay ya puedes empezarlos;
- renderer de mundo todavia necesita varios capitulos mas.

## Donde tocar manana segun lo que quieras anadir

### Quiero cambiar el triangulo por otra forma simple

Si sigue siendo algo muy pequeno y provisional, puedes tocar:

- `shader.vert` para cambiar posiciones;
- `create_command_buffers` para cambiar `cmd_draw` si dibujas mas vertices;
- `compile.bat` para recompilar a SPIR-V.

Pero esto deja de escalar muy rapido. En cuanto quieras una figura seria, entra en vertex buffers.

### Quiero una figura 3D real

Tu siguiente bloque importante es:

- capitulo 22: descripcion de vertex input;
- capitulo 23: creacion de vertex buffer;
- capitulo 24: staging buffer;
- capitulo 25: index buffer;
- capitulo 26 y 27: uniforms y descriptor sets;
- capitulo 31: depth buffering.

Zonas del codigo que tocaras:

- nuevo struct `Vertex` en Rust;
- `create_pipeline` para declarar vertex input;
- nuevas funciones de buffers;
- `create_command_buffers` para bind de vertex/index buffers;
- `shader.vert` para leer atributos reales;
- `cmd_draw` -> probablemente `cmd_draw_indexed`.

### Quiero texturas

Necesitaras:

- capitulo 28: images;
- capitulo 29: image view y sampler;
- capitulo 30: combined image sampler.

Tocaras:

- nuevas funciones de carga y subida de textura;
- image transition/copy logic;
- descriptor set layout;
- pipeline layout;
- fragment shader para samplear textura.

### Quiero controles y movimiento

No hace falta esperar a texturas para esto.

Puedes empezar tocando:

- `main` para capturar `WindowEvent::KeyboardInput`, raton y resize;
- una nueva estructura `GameState` o `InputState`;
- una fase `update` antes del render;
- mas adelante, un `Uniform Buffer` para enviar matrices de camara a GPU.

Recomendacion fuerte:

- no metas logica de jugador dentro de helpers Vulkan;
- el renderer debe consumir estado del juego, no definirlo.

### Quiero chunks

Los chunks pertenecen primero al dominio del juego, no al bootstrap Vulkan.

Empieza por:

- definir `Block`, `ChunkCoord`, `Chunk`, `World`;
- decidir el formato de almacenamiento;
- decidir como marcar caras visibles;
- generar malla visible por chunk en CPU.

Despues, el renderer consumira esa malla mediante:

- vertex buffers;
- index buffers;
- depth buffer;
- regrabado o reciclado de command buffers.

### Quiero movimiento de camara y proyeccion

La progresion sana es:

1. input y estado de camara en CPU;
2. matrices `model/view/projection` en CPU;
3. `Uniform Buffer` por frame;
4. descriptor sets;
5. vertex shader leyendo esas matrices.

Ese es el punto en el que dejas atras el "triangulo fijo" y empiezas a tener una escena.

## Siguiente ruta recomendada

Si quieres avanzar con el menor dolor posible, esta secuencia es muy razonable:

1. cerrar bien el capitulo 21:
   anadir manejo explicito de `WindowEvent::Resized` y estado `resized`.
2. entrar en 22-25:
   vertex buffer, staging buffer e index buffer.
3. entrar en 26-27:
   uniform buffer y descriptor sets para camara y transformaciones.
4. entrar en 28-30:
   texturas.
5. entrar en 31:
   depth buffering.
6. entonces ya empezar con:
   figuras 3D, cubos, chunk meshing, movimiento de camara y mundo.

La idea no es seguir capitulos por cumplirlos, sino porque cada uno desbloquea la siguiente capa real del juego.

## Reglas de arquitectura para no ensuciar el proyecto

Estas reglas te van a ahorrar mucho sufrimiento cuando empieces a mezclar gameplay con renderer:

- el mundo debe existir aunque manana cambies Vulkan por otro backend;
- el renderer debe recibir datos del mundo, no decidir la logica del mundo;
- input actualiza estado del jugador, no objetos Vulkan directamente;
- `main.rs` ya es demasiado grande para convertirse en renderer + juego + assets + input + mundo;
- antes de crecer mucho mas, convendra separar por modulos al menos: `renderer`, `swapchain`, `pipeline`, `sync`, `game_state`, `input`.

## Checklist rapido para no olvidar nada cuando toques algo

### Si cambias shaders

- recompila `vert.spv` y `frag.spv`;
- confirma que el pipeline sigue esperando lo mismo;
- confirma que `cmd_draw` sigue coincidiendo con la geometria esperada.

### Si cambias el orden de vertices

- revisa `front_face`;
- revisa `cull_mode`.

### Si cambias el tamano o comportamiento de ventana

- revisa `get_swapchain_extent`;
- revisa `recreate_swapchain`;
- considera anadir `WindowEvent::Resized`.

### Si pasas a geometria dinamica

- revisa `create_command_pool`;
- revisa cuando y como regrabar command buffers;
- revisa sincronizacion por frame y por imagen.

## Resumen final

Has llegado al punto en el que Vulkan deja de ser solo bootstrap y empieza a poder sostener un renderer de juego.

Lo que existe hoy no es todavia un renderer de sandbox, pero si una base seria para construirlo. El siguiente gran salto no es "mas triangulos", sino sustituir datos hardcodeados por datos reales:

- vertices reales;
- indices reales;
- transformaciones reales;
- texturas reales;
- profundidad real;
- estado de juego real.

Cuando eso ocurra, este triangulo dejara de ser una demo y pasara a ser el primer ladrillo de tu juego.

## Enlaces internos

- [[Proyecto - Sandbox con Vulkan]]
- [[Capitulo 07 - Paquetes, crates y modulos]]
- [[Capitulo 09 - Manejo de errores]]
- [[Concepto - Structs]]
- [[Concepto - Modulos]]
- [[Concepto - Result]]
