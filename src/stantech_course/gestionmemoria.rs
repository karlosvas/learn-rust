pub fn main(){
    test_string();
    prueba_pedirdatos();
}

fn prueba_pedirdatos() {
    let mut name: String = String::new();
    println!("Enter your name :");
    std::io::stdin().read_line(&mut name).unwrap();
    let x: usize = aqui_referencia(&name);
    // add_to_string(&mut name);
    
    let name = name.trim(); // shadowing
    println!("Tu nombre es {name} tiene caracteres {x}");
}

fn test_string(){
    let hola: &str = "hola";
    println!("{}", hola);
    // Hola deja de estar en uso y se elimina de memoria

    crear_string();

    let s: String = String::from("PruevaOwnership");
    nuevo_denno(s); // Aqui se hace un move;

    let x: i32 = 5;
    aqui_copio(x); // Aqui se hace un copy
    println!("{}", x);

    // Esto daria error
    let sy: String = String::from("sy");
    let sc = muevo_return(sy); // Sy desaparece ya no podriamos utilizarlo
    // println!("{}", sy);

    let sa: String =  String::from("sy");
    let sa_len: usize = aqui_referencia(&sa);
    println!("La String {} tiene {} caracteres", sa, sa);

    let mut sv: String = String::from("Hasta ");
    add_to_string(&mut sv);
    println!("{}", sv);

}

fn nuevo_denno(string: String){
    println!("{}", string); // Se hace un move
    // string se livera, S ya se movio no existe mas
}

fn aqui_copio(i: i32){
    println!("{}", i); // Se hace un copy
}

fn muevo_return(string: String) -> String {
    string
}

fn aqui_referencia(string: &String) -> usize {
    string.len()
}

fn add_to_string(s: &mut String) {
    s.push_str("nunca");
}

fn crear_string(){
    let s = String::from("Holaa"); // Creamos una string
    let mut s = String::from("Adios"); // Xcreamos un string mutable;

    // Move
    let var1 = 1;
    let var2 = var1;
    // Aqui esto se copia, en cambio para un string
    let s1 = String::from("s1");
    let s2 = s1.clone();
    println!("{}", s2);
}