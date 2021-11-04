use std::io;

fn main() {

    let a_sentence: &str = "Une belle phrase.";

    let the_first_word = first_word(&a_sentence);

    println!("Le premier mot est: {}",the_first_word);

    println!("================");
    println!("String reverser!");
    println!("================");

    println!("Entrez quelquechose...");
    let mut the_string = String::new();

    io::stdin()
        .read_line(&mut the_string)
        .expect("!!!");

    let the_string: String = the_string
        .trim()
        .parse()
        .ok()
        .expect("!!!");

    let result = string_reverse(the_string);

    println!("Vous avez écrit: \"{}\"",result);


}

fn string_reverse(the_string: String) -> String{
    println!("Taille du tableau: {}",the_string.len());
    let mut i = 0;
    while i < the_string.len(){
        println!("i: {:?}",i);
        i += 1;
    }
    the_string
}

fn first_word(s: &str) -> usize {
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' '{
            return i;
        }
    }
    s.len()
}
