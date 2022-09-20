pub mod buscaminas;

use std::env;
use std::fs;

use crate::buscaminas::Buscaminas;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let ruta_archivo = &args[1]; // El argumento 1 del programa debe indicar el path del archivo (el argumento 0 es el nombre del archivo ejecutado)
    procesar_buscaminas(ruta_archivo);
}

pub fn procesar_buscaminas(ruta_archivo: &String) {
    let contenido_archivo = fs::read_to_string(ruta_archivo).expect("Error abriendo el archivo");
    let mut buscaminas = Buscaminas::from(contenido_archivo);
    buscaminas.resolver();
    println!("{}", buscaminas);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buscaminas_funciona_correctamente() {
        procesar_buscaminas(&"data/input.txt".to_string());
        // Should not panic
    }
}
