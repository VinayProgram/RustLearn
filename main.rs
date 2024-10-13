use std::io;
fn main(){
    println!("guess the number");
    let mut gues=String::new();
    io::stdin().read_line(&mut gues).expect("failed to read line");
    println!("gues:{}",gues);
}