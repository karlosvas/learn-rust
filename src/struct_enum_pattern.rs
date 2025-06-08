
// Estructura de datos, tipo
#[derive(Debug)]
struct suscriptor_de_karlosvas {
    nombre: String,
    beautiful: bool,
    es_inteligente: bool
}

// Constructor
// fn se_suscribe(nuevo_suscriptor: String) -> suscriptor_de_karlosvas {
//     suscriptor_de_karlosvas {
//         es_inteligente: true,
//         nombre: nuevo_suscriptor,
//         beautiful: true
//     }
// } 
// Esto es lo mismo
fn se_suscribe(nombre: String) -> suscriptor_de_karlosvas {
    suscriptor_de_karlosvas { es_inteligente: true, nombre, beautiful: true }
} 

impl suscriptor_de_karlosvas {
    fn se_desuscribe(&mut self, nuevo_nombre: String) {
        self.nombre = nuevo_nombre;
        self.beautiful = false;
        self.es_inteligente = false;
    }
    // Funcion asociada
    fn new_sub(nuevo_suscriptor: String) -> suscriptor_de_karlosvas {
        suscriptor_de_karlosvas {
            es_inteligente:  true,
            nombre: nuevo_suscriptor,
            beautiful: true
        }
    }
}

// Custom lifetimes(Cuando no eres el peopietario)
// Mientras exista una instancia de User, las referencias (&'a str) que contiene deben seguir siendo válidas (no deben ser eliminadas ni modificadas)
struct User<'a> {
    username: &'a str,
    email: &'a str,
    sing_count: u64,
    active: bool
}

struct RGBColor(i32, i32, i32); // Estructura a modo de tupla


// Enum
pub enum Vehiculo{
    Combustion (Coche),
    Hibrido (Coche),
    Elecronico (Coche)
}

pub struct Coche {
    caballos: i32,
    marca: String,
    modelo: String,
    coset: i32
}

pub fn main(){
    // Creammo sun nuevo suscriptor
    let nuevo_suscriptor: suscriptor_de_karlosvas = se_suscribe(String::from("Stan"));
    print!("{}", nuevo_suscriptor.nombre);
    // Todos los campos no especificados los hereda del suiscriptor que se le pasa
    let mut segundo_sub = suscriptor_de_karlosvas {
        nombre: String::from("Stana2"),
        ..nuevo_suscriptor
    };
    segundo_sub.se_desuscribe(String::from("Desterrado"));
    print!("{}", segundo_sub.nombre);

    let rgb: RGBColor = RGBColor(233, 233, 233);
    println!("{}", rgb.0);

    let hola: suscriptor_de_karlosvas = suscriptor_de_karlosvas::new_sub(String::from("hola"));
    println!("{:?}", hola);

    let fav_num: Option<i8> = Some(24);
    // let x = 5+fav_num.unwrap();
    match fav_num {
        Some(val) => print!("Mi numero favorito es = {}", val),
        None => println!("No hay ningun valor")
    }

    // Unwrap
    let mut name: String = String::new();
    let size = std::io::stdin().read_line(&mut name);
    match size {
        Ok(val) => {println!("Este es el tamaño de tu nombre = {}", val)}
        Err(_) => println!("Error")
    }
    if let Some(i) = fav_num {
        println!("Este es mi numero favorito = {}", i);
    }  else{
        println!("No hay ningun valor");
    }

    // Life time scope, se pueden crear bloques de codigo para  reducir el tiempo mde vida o ailsarlo de el resto de codigo.
    println!("Antes del bloque");
    {
        println!("Dentro del bloque");
        let x = 5;
        println!("x = {}", x);
    }
    println!("Después del bloque");
}