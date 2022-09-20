use std::cmp::max;
use std::cmp::min;
use std::env;
use std::fs;

static VACIO : u8 = b'.';
static MINA : u8 = b'*';
static CERO : u8 = b'0';

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let ruta_archivo = &args[1]; // El argumento 1 del programa debe indicar el path del archivo (el argumento 0 es el nombre del archivo ejecutado)
    procesar_buscaminas(ruta_archivo);
}

pub fn procesar_buscaminas(ruta_archivo: &String) {
    let contenido_archivo = fs::read_to_string(ruta_archivo).expect("Error abriendo el archivo");
    resolver_buscaminas(contenido_archivo);
}

fn resolver_buscaminas(file_read: String) {
    let input_bytes = file_read.as_bytes();
    let mut buscaminas = armar_buscaminas(input_bytes);
    buscar_minas_adyacentes(&mut buscaminas);
    imprimir_tablero(&buscaminas);
}

fn armar_buscaminas(input_bytes: &[u8]) -> Vec<Vec<u8>> {
    let mut buscaminas = Vec::<Vec<u8>>::new();
    let mut fila = 0;
    let vector = Vec::<u8>::new();
    buscaminas.push(vector);
    for &byte in input_bytes {
        match byte {
            10 => {
                // 10 = ASCII para LF
                fila = fila + 1; // Un salto de línea significa una nueva fila
                let vector = Vec::<u8>::new();
                buscaminas.push(vector);
            }
            42 => buscaminas[fila].push(MINA), // 42 = ASCII para .
            46 => buscaminas[fila].push(VACIO), // 46 = ASCII para *
            _ => panic!("Caracter inválido en el buscaminas"),
        }
    }
    buscaminas
}

fn buscar_minas_adyacentes(buscaminas: &mut Vec<Vec<u8>>) {
    let fila_size = buscaminas.len();
    let columna_size = buscaminas[0].len();
    for fila in 0..fila_size {
        for columna in 0..columna_size {
            if es_una_mina(&buscaminas[fila][columna]) {
                incrementar_adyacentes(buscaminas, fila, columna);
            }
        }
    }
}

fn es_una_mina(casilla: &u8) -> bool {
    *casilla == MINA
}

fn incrementar_adyacentes(buscaminas: &mut Vec<Vec<u8>>, fila: usize, columna: usize) {
    // Casteo a isize porque las restas pueden dar -1
    let fila_desde = max(0, fila as isize - 1) as usize;
    let fila_hasta = min(buscaminas.len() - 1, fila + 1) + 1;
    let columna_desde = max(0, columna as isize - 1) as usize;
    let columna_hasta = min(buscaminas[fila].len() - 1, columna + 1) + 1;
    // Recorro el cuadrado que redodea la mina, cuidadosamente de no salir de los límites
    for f in fila_desde..fila_hasta {
        for c in columna_desde..columna_hasta {
            let casilla = buscaminas
                .get_mut(f)
                .expect("Error inesperado accediendo a una fila del buscaminas")
                .get_mut(c)
                .expect("Error inesperado accediendo a una columna del buscaminas");
            if !es_una_mina(casilla) {
                incrementar_casilla(casilla);
            }
        }
    }
}

fn incrementar_casilla(casilla: &mut u8) {
    if *casilla == VACIO {
        //Inicializar casillas adyacentes aún no inicializadas
        *casilla = CERO;
    }
    *casilla = *casilla + 1;
}

fn imprimir_tablero(tablero: &Vec<Vec<u8>>) {
    for fila in tablero {
        for columna in fila {
            print!("{} ", char::from(*columna));
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arma_buscaminas_en_base_a_bytes() {
        let input_bytes: &[u8] = &vec![42, 46, 46, 10, 46, 46, 42, 10, 46, 42, 46];
        let buscaminas = armar_buscaminas(input_bytes);
        assert_eq!(buscaminas.len(), 3);
        assert_eq!(buscaminas[0].len(), 3);
        assert_eq!(buscaminas[0][0], MINA);
        assert_eq!(buscaminas[0][1], VACIO);
        assert_eq!(buscaminas[0][2], VACIO);
        assert_eq!(buscaminas[1][0], VACIO);
        assert_eq!(buscaminas[1][1], VACIO);
        assert_eq!(buscaminas[1][2], MINA);
        assert_eq!(buscaminas[2][0], VACIO);
        assert_eq!(buscaminas[2][1], MINA);
        assert_eq!(buscaminas[2][2], VACIO);
    }

    #[test]
    #[should_panic]
    fn falla_si_hay_un_caracter_invalido() {
        let input_bytes: &[u8] = &vec![41];
        armar_buscaminas(input_bytes);
    }

    #[test]
    fn busca_minas_adyacentes_correctamente() {
        let mut buscaminas = vec![
            vec![MINA, VACIO, VACIO],
            vec![VACIO, VACIO, MINA],
            vec![VACIO, MINA, VACIO],
        ];
        buscar_minas_adyacentes(&mut buscaminas);
        assert_eq!(buscaminas[0][0], MINA);
        assert_eq!(buscaminas[0][1], b'2');
        assert_eq!(buscaminas[0][2], b'1');
        assert_eq!(buscaminas[1][0], b'2');
        assert_eq!(buscaminas[1][1], b'3');
        assert_eq!(buscaminas[1][2], MINA);
        assert_eq!(buscaminas[2][0], b'1');
        assert_eq!(buscaminas[2][1], MINA);
        assert_eq!(buscaminas[2][2], b'2');
    }

    #[test]
    fn incrementa_adyacentes_correctamente() {
        let mut buscaminas = vec![
            vec![MINA, VACIO, VACIO],
            vec![VACIO, VACIO, MINA],
            vec![VACIO, MINA, VACIO],
        ];
        let fila = 2;
        let columna = 1;
        incrementar_adyacentes(&mut buscaminas, fila, columna);
        assert_eq!(buscaminas[0][0], MINA);
        assert_eq!(buscaminas[0][1], VACIO);
        assert_eq!(buscaminas[0][2], VACIO);
        assert_eq!(buscaminas[1][0], b'1');
        assert_eq!(buscaminas[1][1], b'1');
        assert_eq!(buscaminas[1][2], MINA);
        assert_eq!(buscaminas[2][0], b'1');
        assert_eq!(buscaminas[2][1], MINA);
        assert_eq!(buscaminas[2][2], b'1');
    }

    #[test]
    fn incrementa_casilla_correctamente() {
        let mut casilla = b'1';
        incrementar_casilla(&mut casilla);
        assert_eq!(casilla, b'2');
    }

    #[test]
    fn inicializa_casilla_en_cero_correctamente_y_la_incrementa() {
        let mut casilla = VACIO;
        incrementar_casilla(&mut casilla);
        assert_eq!(casilla, b'1');
    }
}
