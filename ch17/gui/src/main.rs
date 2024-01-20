use gui::{ Draw, Screen, Button, TextField, SelectBox };

struct Image {
    width: u32,
    height: u32,
    src: String,
}
impl Draw for Image {
    fn draw(&self) {
        // ...
    }
}

fn main() {
    // ...
}

