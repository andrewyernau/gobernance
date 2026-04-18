---
tags:
  - rust
  - rust/book
  - rust/ch08
status: completado
---

# Capitulo 08 - Colecciones comunes

## Idea central

Empezar a trabajar con datos reales que crecen, cambian y necesitan acceso eficiente sin perder el control de ownership.

## Subcapitulos

- 8.1 Almacenando listas de valores con vectores -> [[Concepto - Vec]]
- 8.2 Almacenando texto UTF-8 con strings -> [[Concepto - String y str]]
- 8.3 Almacenando claves y valores con hash maps -> [[Concepto - HashMap]]

## Codigo del repo

- `learning/book/ch08/vectors`
- `learning/book/ch08/strings`
- `learning/book/ch08/hashmaps`

## Ruta recomendada

1. Domina `Vec<T>` porque sera la base para listas, buffers, chunks y colecciones de entidades.
2. Pasa a `String` y `&str` para fijar la diferencia entre texto prestado, texto poseido y UTF-8.
3. Cierra con `HashMap` cuando el problema ya no es posicion sino acceso por clave.
4. Relee el capitulo pensando en ownership dentro de colecciones: ese es el aprendizaje que luego mas pesa.

## Por que importa

Sin este capitulo, `Cubix` acabaria pronto con:

- listas mal modeladas para bloques y vertices;
- errores al mover `String` o guardar referencias invalidas;
- `HashMap` usados sin entender ownership ni `entry`;
- decisiones pobres entre acceso por indice y acceso por clave.

## Error tipico

- usar `v[i]` cuando `get` te daria una via segura;
- intentar indexar `String` como si fuese un array de caracteres;
- asumir que `HashMap` conserva orden;
- meter referencias en colecciones sin pensar si los datos viviran lo suficiente.

## Enlaces

- [[Capitulo 07 - Paquetes, crates y modulos]]
- [[Capitulo 09 - Manejo de errores]]
- [[Concepto - Vec]]
- [[Concepto - String y str]]
- [[Concepto - HashMap]]
- [[Concepto - Ownership]]
- [[Concepto - Borrowing y referencias]]
- [[Proyecto - Sandbox con Vulkan]]
