use std::error::Error;
use std::io;
use std::num::ParseIntError;

fn main() -> Result<(), Box<dyn Error>> {
    require_valid_chunk_size(16);

    match parse_render_distance("far") {
        Ok(distance) => println!("Render distance valida: {distance}"),
        Err(error) => println!("Error recuperable al leer render distance: {error}"),
    }

    let render_distance = parse_render_distance("12")?;
    let spawn_height = load_spawn_height(&[0, 0, 1, 1])?;

    println!("Render distance aplicada: {render_distance}");
    println!("Altura de spawn encontrada: {spawn_height}");
    println!("Descomenta una llamada invalida a require_valid_chunk_size para ver un panic!");

    Ok(())
}

/// Un panic es razonable cuando una invariante interna se rompe.
fn require_valid_chunk_size(chunk_size: u32) {
    if chunk_size == 0 {
        panic!("chunk_size debe ser mayor que 0");
    }
}

fn parse_render_distance(input: &str) -> Result<u32, ParseIntError> {
    input.trim().parse()
}

fn load_spawn_height(column: &[u8]) -> Result<usize, io::Error> {
    first_solid_block_height(column)
        .ok_or_else(|| io::Error::other("No hay bloque solido para spawnear"))
}

fn first_solid_block_height(column: &[u8]) -> Option<usize> {
    column.iter().position(|&block| block != 0)
}
