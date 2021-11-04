use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");


    let mut v: Vec<u64> = Vec::new();

//    while num > 0{
//        v.push(num as u64 % 2);
//        if num%2 == 1 {
//            num = (num - 1) / 2;
//        }else{
//            num = num/2;
//        }
//    } 

    // decimal to binary
    let i = 9;
    println!("i = {:b}",i);

    let mut num_bin: Vec<u64> = Vec::new();
    let mut i: usize = v.len();

    while i > 0{
        num_bin.push(v[i - 1]);
        i -= 1;
    }

    println!("{}", vec_to_string(&num_bin));
}


fn vec_to_string(vec: &[u64]) -> String {
    let mut result = String::new();

    for num in vec {
        result.push_str(&num.to_string());
    }
    result
}

