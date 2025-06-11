use self::List::{Cons, Nil};

#[derive(Debug)]
enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

#[derive(Debug)]
enum List2 {
    Cons(i32, Box<List2>),
    Nil,
}

trait PrettyPrint {
    fn pretty_print(&self) -> String;
}

impl<T: std::fmt::Debug> PrettyPrint for List<T> {
    fn pretty_print(&self) -> String {
        match self {
            List::Cons(head, tail) => format!("{:?}, {}", head, tail.pretty_print()),
            List::Nil => "NIL".to_string(),
        }
    }
}

impl PrettyPrint for List2 {
    fn pretty_print(&self) -> String {
        match self {
            List2::Cons(head, tail) => format!("{}, {}", head, tail.pretty_print()),
            List2::Nil => "NIL".to_string(),
        }
    }
}

// Diferencia netre null y nil
// Null: es un valor que no apunta a nada, es un valor que no existe.
// Nil: Voluntariamente tu siendo conocedor estas diciendo que no haya valor, es un valor que no existe porque tu lo has decidido.

use std::ops::Deref;

struct MiPuntero<T>(T);

// MiPuntero es una estructura que contiene un valor de tipo T
// Esta estructura se comporta como un puntero, pero no es un puntero en sí mismo
impl<T> MiPuntero<T> {
    fn new (value: T) -> MiPuntero<T> {
        MiPuntero(value)
    }
}

// Dref es un trait que permite acceder al valor dentro de un puntero
// En este caso, estamos implementando el trait Deref para que podamos acceder al valor dentro de MiPuntero como si fuera un puntero normal
impl<T> Deref for MiPuntero<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

// Asegurarnos de que se haya eliminado el puntero
// Cuando se elimina un puntero, se ejecuta el destructor
impl<T> Drop for MiPuntero<T> {
    fn drop(&mut self) {
        println!("Adios mundo cruel");
    }
}

pub fn main(){
    let b = Box::new(5);
    println!("El valor de b es: {}", b);

    let _list = Cons(1, Box::new(Cons(2, Box::new(List::Nil))));
    let _list2 = List2::Cons(1, Box::new(List2::Cons(2, Box::new(List2::Nil))));

    println!("{:?}", _list);
    println!("{}", _list.pretty_print());
    println!("{:?}",_list2);
    println!("{}", _list2.pretty_print());

    // Deferenciar un puntero
    let a = 24;
    let b = MiPuntero::new(a);
    assert_eq!(24, a);
    assert_eq!(24, *b);

    // Reference counter para que siga vivo meintras haya almenos algun puntero que lo apunte 
    // Un reference counter es un contador de referencias que mantiene un recuento de cuántas referencias hay a un valor en memoria.
    let _y = Cons(23, Box::new(Cons(25, Box::new(Nil))));
    println!("El valor de y es: {:?}", _y);
}