fn shadowing(){
    let x: i32 = 1;
    let x: i32 = x+23;
    let x: i32 = x-23/23;
    println!("El resultado es x = {}", x);
}

fn scalars(){
    // Enteros
    let ocho_bits: i8 = 1;
    let diezseis_bits: i16 = 1;
    let sesentaycuatro_bits: i64 = 1;
    let unodsoocho_bits: i128 = 1;

    // Igual con los floats y tenemos operaciones matemáticas.
    let float_tresdos: f32 = 2.65;
    let float_data: f32 = float_tresdos + 23.54;
    let variable: bool = true;
    let my_char: char = 'a';

    print!("{} {} {} {} {} {} {} {}", ocho_bits, diezseis_bits, sesentaycuatro_bits, unodsoocho_bits, float_data, float_tresdos, variable, my_char)
}


fn compoud() -> i32 {
    // Tipos compuestos
    // Tuplas
    // let tup: (i32,f64,char) = (12 , 3.45, 'b');

    // Array
    let arr: [i32; 4] = [1,2,3,4]; 
    // let val: i32 = arr[0]; // Primer elemento del array
    // let val2: i32 = arr[4]; // Explota

    for i in arr.iter(){
        println!("{}", i);
    } 

    let mut ent: i32 = 0;

    while ent<10 {
        ent = ent + 1;
    }

    // Como un while true
    loop {
        ent = ent - 10;
        break;
    }

    ent
}

fn calcula_factorial(number: u128) -> u128 {
    if number == 0 || number == 1{
        1
    } else{
        let mut result: u128 = number;
        for i in (1..number).rev() {
            result = result * i;
        }
        result
    }
}

fn is_prime(number: u128) -> bool {
    if number < 2{
        false
    } else {
        let mut prime: bool = true;
        // Casteo a float
        let squere: f64 = number as f64;

        // Recorremos de a la raiz cuadrada
        for i in 2..((squere.sqrt() as i128)+1) {
            if (number as i128)%i == 0 {
                prime = false;
                break;
            }
        }

        prime
    }
}

pub fn main(){
    shadowing();
    scalars();
    let sol: i32 = compoud();
    println!("{}", sol);
    let resultado = calcula_factorial(1);
    let resultado2 = calcula_factorial(3);
    println!("{} {}", resultado, resultado2);
    println!("{}", is_prime(12));
    println!("{}", is_prime(13));
    println!("{}", is_prime(15));
    println!("{}", is_prime(0));
    println!("{}", is_prime(1));
    println!("{}", is_prime(2));
}