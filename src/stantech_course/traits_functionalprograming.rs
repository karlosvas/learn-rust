pub trait PrettyPrint {
    fn pretty_print(&self) -> String;
}

pub struct Suscriptor {
    pub nombre: String,
    pub inteligente: bool,
    pub hermoso: bool
}

impl PrettyPrint for Suscriptor  {
    fn pretty_print(&self) -> String {
        format!("El suscriptor ese llama {} , es muy {} y {}",
            self.nombre,
            if self.inteligente {"inteligente"} else {"tonto"},
            if self.hermoso {"hermoso"} else {"feisimo"}
        )
    }
}

fn middle_element<T: Copy + PrettyPrint>(list: &[T]) -> T {
    list[list.len() / 2]
}

// fn where_middle_element<T, U>(t: &[T], u: &[U]) -> (T, U) where T: Copy + PrettyPrint,U: PrettyPrint, {
//     (t[0], u[0])
// }

fn super_long_calculation(parameter: i8) -> i8 {
    println!("Super long calculation ⌛⏳");
    parameter
}

// Fn viene por defecto en rust y es para funciones que se pueden llamar
// FnMut es para funciones que pueden ser llamadas y modificadas
struct MejoraCalculo<T> where T: Fn(u32) -> u32 {
    operation: T,
    value: Option<u32>,
}

// Cacher
impl<T> MejoraCalculo<T> where T: Fn(u32) -> u32 {
    fn new(operation: T) -> Self {
        MejoraCalculo {
            operation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        // if let Some(value) = self.value {
        //     value
        // } else {
        //     let result = (self.operation)(input);
        //     self.value = Some(result);
        //     result
        // }
        match self.value {
            Some(value) => value,
            None => {
                let result = (self.operation)(arg);
                self.value = Some(result);
                result
            }
        }
    }
}


// Iteracores
fn iterators(){
    let v1 = vec![1, 2, 3, 4, 5];

    println!();
    for number in v1.iter() {
        print!("{} ", number);
    }
    println!();

    let result: Vec<_> = v1.iter()
        .map(|x| x +1)
        .collect();

    print!("{:?}", result);
}


pub fn main() {
    let suscriptor1: Suscriptor = Suscriptor {
        nombre: String::from("Juan"),
        inteligente: true,
        hermoso: false,
    };

    let suscriptor2: Suscriptor = Suscriptor{
        nombre: "Pedro".to_owned(),
        inteligente: false,
        hermoso: true
    };

    println!("{}", suscriptor1.pretty_print());
    println!("{}", suscriptor2.pretty_print());

    // let a: Suscriptor = middle_element();

    // Closures
    fn calculationfn(number: i8) -> i8 {
        super_long_calculation(number)
    }
    
    let calculation = |number: i8| -> i8 {
        super_long_calculation(number)
    };
    print!("{}", calculation(4));

    let variable = 5;
    let captura = |z: i8| z == variable;
    assert!(captura(5));     // Prueba la closure

    iterators();
}