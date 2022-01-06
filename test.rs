use std::str;

fn main(){
    let num = str::parse::<f32>("34").unwrap();
    println!("parsed number {}",num)
}
