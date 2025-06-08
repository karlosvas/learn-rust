use rand::prelude::*;
use std::fs::File;

fn numero_random() {
    let mut rng: ThreadRng = thread_rng();
    let y: f64 = rng.gen();
    println!("{}", y);
}

fn panic_moment(){
    let f = File::open("no_existe.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("El archivo no existe, a ver si buscas mejor {:?}", error);
        }
    };
}

mod taller {
    pub mod recepcion{
        fn add_cita(){}
        fn llevar_a_taller(){}
        // super::traer_grua();
    }
    pub mod garaje{
        pub fn arreglar_coche(){}
    }
}

pub fn llevar_a_arreglar(){
    // "crate" es como el slash en una ruta absoluta
    crate::taller::garaje::arreglar_coche();
    // SI quiero utilizar rutas relativas
    taller::garaje::arreglar_coche();
}

// El termino USE
use crate::taller::garaje as my_garaje;
pub fn prueba_codigo(){
    my_garaje::arreglar_coche();
}

// Modo idiomatico
use crate::taller::garaje::arreglar_coche;
pub fn prueba_codigo2(){
    arreglar_coche();
}

// use mechainchop::garaje2::arreglar_coche2;
// use tallermecanico::mi_taller::arreglar_super_coche;

pub fn main(){
    numero_random();
}