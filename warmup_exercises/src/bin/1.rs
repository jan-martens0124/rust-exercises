//TODO fix this programs in two ways

// fn main() {
//     let s = String::from("hello, world");

//     let clone_of_s = s.clone();
//     print_str(clone_of_s);

//     println!("{}", s);
// }

// //keep this function
// fn print_str(arg: String) {
//     println!("{}", arg)
// }

fn main() {
    let s = String::from("hello, world");

    print_str(&s);

    

    println!("{}", s);
}

//keep this function
fn print_str(arg: &String) {
    println!("{}", arg)
}