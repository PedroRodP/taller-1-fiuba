use std::cmp::max;
use std::cmp::min;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];   // El argumento 1 del programa debe indicar el path del archivo (el argumento 0 es el nombre del archivo ejecutado)
    let content = fs::read_to_string(file_path)
        .expect("Error leyendo el archivo");
    let input_bytes = content.as_bytes();
    let mut buscaminas = armar_buscaminas(input_bytes);
    let tablero = buscar_minas_adyacentes(&mut buscaminas);
    imprimir_tablero(tablero);
}

fn imprimir_tablero(tablero : &Vec::<Vec::<String>>) {
    for fila in tablero {
        for columna in fila {
            print!("{} ", columna)
        }
        println!()
    }
}

fn buscar_minas_adyacentes(buscaminas : &mut Vec::<Vec::<String>>) -> &mut Vec::<Vec::<String>> {
    let fila_size = buscaminas.len();
    let columna_size = buscaminas[0].len();
    for fila in 0..fila_size {
        for columna in 0..columna_size {
            if buscaminas[fila][columna] == "*" {
                incrementar_adyacentes(buscaminas, fila, columna);
            }
        }
    }
    buscaminas
}

fn incrementar_adyacentes(buscaminas : &mut Vec::<Vec::<String>>, fila : usize, columna : usize) {
    // Casteo a isize porque las restas pueden dar -1
    let fila_desde = max(0, fila as isize - 1) as usize;
    let fila_hasta = min(buscaminas.len() - 1, fila + 1) + 1;
    let columna_desde = max(0, columna as isize - 1) as usize;
    let columna_hasta = min(buscaminas[fila].len() - 1, columna + 1) + 1;
    // Recorro el cuadrado que redodea la mina, cuidadosamente de no salir de los límites
    for f in fila_desde..fila_hasta {
        for c in columna_desde..columna_hasta {
            let casilla = buscaminas.get_mut(f).unwrap().get_mut(c).unwrap();
            if *casilla != "*".to_string() {
                if *casilla == ".".to_string() { //Inicializar casillas adyacentes aún no inicializadas
                    *casilla = "0".to_string();
                }
                let mut casilla_int : usize = casilla.parse().unwrap();
                casilla_int = casilla_int + 1;
                *casilla = casilla_int.to_string();
            }
        }
    }
}

fn armar_buscaminas(input_bytes: &[u8]) -> Vec::<Vec::<String>> {
    let mut buscaminas = Vec::<Vec::<String>>::new();
    let mut fila = 0;
    let vector = Vec::<String>::new();
    buscaminas.push(vector);
    for &byte in input_bytes {
        match byte {
            10 => { // 10 = ASCII para LF
                fila = fila + 1; // Un salto de línea significa una nueva fila
                let vector = Vec::<String>::new();
                buscaminas.push(vector);
            },
            42 => buscaminas[fila].push("*".to_string()), // 42 = ASCII para .
            46 => buscaminas[fila].push(".".to_string()), // 46 = ASCII para *
            _ => panic!("Caracter inválido en el buscaminas")
        }
    }
    buscaminas
}
