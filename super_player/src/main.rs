mod media;

use media::Playable;

struct Audio(String);
struct Video(String);

impl Playable for Audio {
    fn play(&self) {
        println!("Now playing: {}", self.0);
    }
}

impl Playable for Video {
    fn play(&self) {
        println!("Now playing: {}", self.0);
    }
}

fn main() {
    println!("Super player");
    let audio = Audio("Hotel California".to_string());
    let video = Video("What If?".to_string());
    audio.play();
    video.play();
    Audio::pause();
}
