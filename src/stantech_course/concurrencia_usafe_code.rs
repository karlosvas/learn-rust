// Multipor producer singel consumer
use std::sync::mpsc;
use std::sync::{Arc, Mutex};
use std::{slice, thread, vec};
use std::time::Duration;

// Unsafe keyword permite realizar operaciones que el compilador no puede garantizar que sean seguras
// En Rust, el uso de unsafe es necesario cuando se trabaja con punteros, referencias y otras operaciones que pueden comprometer la seguridad de la memoria
fn usafe_thread(){
    let counter = Arc::new(Mutex::new(0)); // Mutex para proteger el acceso al contador
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // Clonamos el Arc para pasar ownership al hilo
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let mut num = counter.lock().unwrap(); // Bloqueamos el mutex para acceder al contador
                *num += 1; // Incrementamos el contador
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap(); // Esperamos a que todos los hilos terminen
    }

    println!("Result: {}", *counter.lock().unwrap()); // Imprimimos el resultado final del contador
}

fn split_at_mut_rust(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid), // Parte izquierda
            slice::from_raw_parts_mut(ptr.add(mid), len - mid) // Parte derecha
        )
    }
}

fn unsafe_code() {
    let mut num = 5; // => i32
    let r1 = &num as *const i32; // => *const i32, solo lectura
    // r1 es un puntero constante, no se puede modificar el valor al que apunta
    let r2 = &mut num as *mut i32;  // => *mut i32, mutable, se puede modificar el valor al que apunta
    // r2 es un puntero mutable, se puede modificar el valor al que apunta

    // unsafe es un bloque que nos permite hacer cosas que el compilador no puede garantizar que sean seguras
    // En este caso, estamos accediendo a punteros sin garantías de seguridad
    unsafe {
        println!("r1 apunta a: {}", *r1);  // Solo lectura
        *r2 = 10;  // Modificación permitida
        println!("Después de modificar r2, num = {}", num);
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = split_at_mut_rust(r, 3); // Dividimos el vector en dos partes mutables
    print!("Parte a: ");
    for i in a {
        print!("{} ", i);
    }
    println!();
    print!("Parte b: ");    
    for i in b {
        print!("{} ", i);
    }
    println!();
}

// Asincronia
pub fn main(){
    // "||" Se utiliza para decir que le das el ownership de las variables al hilo
    let handle = thread::spawn( || {
        for i in 1..10 {
            println!("Hi from child thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("Hi from main thread {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // Espera a que el hilo hijo termine


    // Gestion de la memoria compartida
    let (tx, rx) = mpsc::channel(); // Canal para enviar mensajes entre hilos
    let tx1 = tx.clone(); // Clonamos el transmisor para poder usarlo en otro hilo
    
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        
        for val  in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("from"),
            String::from("you"),
        ];
        
        for val  in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(2));
        }
    });

    for recived in rx {
        println!("Get: {:?}", recived);
    }
    
    usafe_thread();
    println!("End of main thread");
    unsafe_code();
}