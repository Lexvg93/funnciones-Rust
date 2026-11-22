fn main() {
    println!("Hello, world!");
    another_function(5);
    print_labeled_measurement(5,'h');
    let x = five();
    println!("The value of x is: {x}");

    //en una expresion el valor de retorno 
    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {y}");

    let a = plus_one(5);
    println!("El valor de a es :{a}");
}

//en rust usamos snake_case que es todo en minuscula y separar palabras por guiones
//al pasar parametros a las funciones se debe declarar el tipo concreto que es
fn another_function(x: i32){
    //println!("Another function");
    println!("The value of x is: {x}");
}

//funcion con 2 parametros
fn print_labeled_measurement(value: i32, unit_label: char){
    println!("The measurement is: {value}{unit_label}");
}

//funcion que retorna un valor de tipo i32
fn five() -> i32 {
    5
}

//funcion con parametros y valor de retorno
fn plus_one(x: i32) -> i32{
    x + 1
}
