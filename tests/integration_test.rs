use ejercicio1::main::procesar_buscaminas;

#[test]
fn buscaminas_funciona_correctamente() {
    procesar_buscaminas(&"data/input.txt".to_string());
    // Should not panic
}
