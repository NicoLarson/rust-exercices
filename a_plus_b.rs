use std::io;

fn main(){
    println!("Somme de A + B");
    println!("Veuillez saisir A: ");

    let mut a = String::new();
    io::stdin()
        .read_line(&mut a)
        .expect("Veuillez saisir un chiffre!");

    let a: f64 = a
        .trim()
        .parse()
        .ok()
        .expect("Invalide!");


    println!("Veuillez saisir B: ");
    let mut b = String::new();
    io::stdin()
        .read_line(&mut b)
        .expect("Veuillez saisir un chiffre!");

    let b: f64 = b
        .trim()
        .parse()
        .ok()
        .expect("Invalide!");

    let result = sum(a,b);

    println!("{} + {} = {}",a,b,result);

}

fn sum(a: f64, b: f64) -> f64{
    let result: f64 = a + b;
    result
}
