use std::cmp::max;
use std::cmp::min;
use std::fmt::Display;

static VACIO : u8 = b'.';
static MINA : u8 = b'*';
static CERO : u8 = b'0';

#[derive (Debug)]
pub struct Buscaminas {
    tablero : Vec<Vec<u8>>
}

impl From<String> for Buscaminas {
    fn from(string : String) -> Buscaminas {
        let mut tablero = Vec::<Vec<u8>>::new();
        let mut fila = 0;
        let columna = Vec::<u8>::new();
        tablero.push(columna);
        for &byte in string.as_bytes() {
            match byte {
                10 => {
                    // 10 = ASCII para LF
                    fila = fila + 1; // Un salto de línea significa una nueva fila
                    let nueva_columna = Vec::<u8>::new();
                    tablero.push(nueva_columna);
                }
                42 => tablero[fila].push(MINA), // 42 = ASCII para .
                46 => tablero[fila].push(VACIO), // 46 = ASCII para *
                _ => panic!("Caracter inválido en el buscaminas"),
            }
        }
        Buscaminas {
            tablero
        }
    }
}

impl Buscaminas {
    pub fn resolver(&mut self) {
        let fila_size = self.tablero.len();
        let columna_size = self.tablero[0].len();
        for fila in 0..fila_size {
            for columna in 0..columna_size {
                if es_una_mina(&self.tablero[fila][columna]) {
                    incrementar_adyacentes(&mut self.tablero, fila, columna);
                }
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

impl Display for Buscaminas {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for fila in &self.tablero {
            for columna in fila {
                print!("{} ", char::from(*columna));
            }
            println!();
        }
        Result::Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arma_buscaminas_en_base_a_string() {
        //let input = &vec![MINA, VACIO, VACIO, LF, VACIO, VACIO, MINA, LF, VACIO, MINA, VACIO];
        let input = "*..\n..*\n.*.".to_string();
        let tablero = Buscaminas::from(input).tablero;
        assert_eq!(tablero.len(), 3);
        assert_eq!(tablero[0].len(), 3);
        assert_eq!(tablero[0][0], MINA);
        assert_eq!(tablero[0][1], VACIO);
        assert_eq!(tablero[0][2], VACIO);
        assert_eq!(tablero[1][0], VACIO);
        assert_eq!(tablero[1][1], VACIO);
        assert_eq!(tablero[1][2], MINA);
        assert_eq!(tablero[2][0], VACIO);
        assert_eq!(tablero[2][1], MINA);
        assert_eq!(tablero[2][2], VACIO);
    }

    #[test]
    #[should_panic]
    fn falla_si_hay_un_caracter_invalido() {
        let input = "..*\n*.,".to_string();
        let _ = Buscaminas::from(input);
    }

    #[test]
    fn resuelve_correctamente() {
        let input = "*..\n..*\n.*.".to_string();
        let mut buscaminas = Buscaminas::from(input);
        buscaminas.resolver();
        let tablero = buscaminas.tablero;
        assert_eq!(tablero[0][0], MINA);
        assert_eq!(tablero[0][1], b'2');
        assert_eq!(tablero[0][2], b'1');
        assert_eq!(tablero[1][0], b'2');
        assert_eq!(tablero[1][1], b'3');
        assert_eq!(tablero[1][2], MINA);
        assert_eq!(tablero[2][0], b'1');
        assert_eq!(tablero[2][1], MINA);
        assert_eq!(tablero[2][2], b'2');
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
