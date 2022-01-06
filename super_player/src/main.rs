struct Audio(String);
struct Video(String);


trait Playable {
    fn play(&self);

    fn pause() {
        println!("Paused!");
    }
}

fn main(){
    println!("Hello World")
}
