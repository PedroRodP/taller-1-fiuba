use std::cmp::max;
use std::cmp::min;
use std::env;
use std::fs;
use std::io::stdin;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let ruta_archivo = &args[1]; // El argumento 1 del programa debe indicar el path del archivo (el argumento 0 es el nombre del archivo ejecutado)
    procesar_buscaminas(ruta_archivo);
}

pub fn procesar_buscaminas(ruta_archivo: &String) {
    let contenido_archivo = fs::read_to_string(ruta_archivo);
    let mut archivo_no_encontrado = true;
    while archivo_no_encontrado {
        solicitar_correccion_archivo(&contenido_archivo, ruta_archivo, &mut archivo_no_encontrado);
    }
    resolver_buscaminas(contenido_archivo.unwrap());
}

fn solicitar_correccion_archivo(
    contenido_archivo: &Result<String, std::io::Error>,
    ruta_archivo: &String,
    archivo_no_encontrado: &mut bool,
) {
    if contenido_archivo.is_err() {
        println!("No se encontró el archivo {}. Revise que se encuentre en la ruta especificada y presione enter para reintentar.", ruta_archivo);
        stdin()
            .read_line(&mut String::new())
            .expect("Error inesperado en stdin");
    } else {
        *archivo_no_encontrado = false;
    }
}

fn resolver_buscaminas(file_read: String) {
    let input_bytes = file_read.as_bytes();
    let mut buscaminas = armar_buscaminas(input_bytes);
    let tablero = buscar_minas_adyacentes(&mut buscaminas);
    imprimir_tablero(tablero);
}

fn armar_buscaminas(input_bytes: &[u8]) -> Vec<Vec<String>> {
    let mut buscaminas = Vec::<Vec<String>>::new();
    let mut fila = 0;
    let vector = Vec::<String>::new();
    buscaminas.push(vector);
    for &byte in input_bytes {
        match byte {
            10 => {
                // 10 = ASCII para LF
                fila = fila + 1; // Un salto de línea significa una nueva fila
                let vector = Vec::<String>::new();
                buscaminas.push(vector);
            }
            42 => buscaminas[fila].push("*".to_string()), // 42 = ASCII para .
            46 => buscaminas[fila].push(".".to_string()), // 46 = ASCII para *
            _ => panic!("Caracter inválido en el buscaminas"),
        }
    }
    buscaminas
}

fn buscar_minas_adyacentes(buscaminas: &mut Vec<Vec<String>>) -> &mut Vec<Vec<String>> {
    let fila_size = buscaminas.len();
    let columna_size = buscaminas[0].len();
    for fila in 0..fila_size {
        for columna in 0..columna_size {
            if es_una_mina(&buscaminas[fila][columna]) {
                incrementar_adyacentes(buscaminas, fila, columna);
            }
        }
    }
    buscaminas
}

fn es_una_mina(casilla: &String) -> bool {
    casilla == "*"
}

fn incrementar_adyacentes(buscaminas: &mut Vec<Vec<String>>, fila: usize, columna: usize) {
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

fn incrementar_casilla(casilla: &mut String) {
    if casilla == "." {
        //Inicializar casillas adyacentes aún no inicializadas
        *casilla = "0".to_string();
    }
    let mut casilla_int: usize = casilla
        .parse()
        .expect("La casilla a incrementar no es numérica ni vacía");
    casilla_int = casilla_int + 1;
    *casilla = casilla_int.to_string();
}

fn imprimir_tablero(tablero: &Vec<Vec<String>>) {
    for fila in tablero {
        for columna in fila {
            print!("{} ", columna)
        }
        println!()
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
        assert_eq!(buscaminas[0][0], "*");
        assert_eq!(buscaminas[0][1], ".");
        assert_eq!(buscaminas[0][2], ".");
        assert_eq!(buscaminas[1][0], ".");
        assert_eq!(buscaminas[1][1], ".");
        assert_eq!(buscaminas[1][2], "*");
        assert_eq!(buscaminas[2][0], ".");
        assert_eq!(buscaminas[2][1], "*");
        assert_eq!(buscaminas[2][2], ".");
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
            vec!["*".to_string(), ".".to_string(), ".".to_string()],
            vec![".".to_string(), ".".to_string(), "*".to_string()],
            vec![".".to_string(), "*".to_string(), ".".to_string()],
        ];
        let resultado = buscar_minas_adyacentes(&mut buscaminas);
        assert_eq!(resultado[0][0], "*");
        assert_eq!(resultado[0][1], "2");
        assert_eq!(resultado[0][2], "1");
        assert_eq!(resultado[1][0], "2");
        assert_eq!(resultado[1][1], "3");
        assert_eq!(resultado[1][2], "*");
        assert_eq!(resultado[2][0], "1");
        assert_eq!(resultado[2][1], "*");
        assert_eq!(resultado[2][2], "2");
    }

    #[test]
    fn incrementa_adyacentes_correctamente() {
        let mut buscaminas = vec![
            vec!["*".to_string(), ".".to_string(), ".".to_string()],
            vec![".".to_string(), ".".to_string(), "*".to_string()],
            vec![".".to_string(), "*".to_string(), ".".to_string()],
        ];
        let fila = 2;
        let columna = 1;
        incrementar_adyacentes(&mut buscaminas, fila, columna);
        assert_eq!(buscaminas[0][0], "*");
        assert_eq!(buscaminas[0][1], ".");
        assert_eq!(buscaminas[0][2], ".");
        assert_eq!(buscaminas[1][0], "1");
        assert_eq!(buscaminas[1][1], "1");
        assert_eq!(buscaminas[1][2], "*");
        assert_eq!(buscaminas[2][0], "1");
        assert_eq!(buscaminas[2][1], "*");
        assert_eq!(buscaminas[2][2], "1");
    }

    #[test]
    fn incrementa_casilla_correctamente() {
        let mut casilla = "1".to_string();
        incrementar_casilla(&mut casilla);
        assert_eq!(casilla, "2");
    }

    #[test]
    fn inicializa_casilla_en_cero_correctamente_y_la_incrementa() {
        let mut casilla = ".".to_string();
        incrementar_casilla(&mut casilla);
        assert_eq!(casilla, "1");
    }
}
