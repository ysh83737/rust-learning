use gui::{ Draw, Screen, Button, TextField, SelectBox };

struct Image {
    width: u32,
    height: u32,
    src: String,
}
impl Draw for Image {
    fn draw(&self) {
        println!("Draw TextField: width={}, height={}, src={}", self.width, self.height, self.src);
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 100,
                height: 20,
                label: String::from("Submit"),
            }),
            Box::new(TextField {
                width: 100,
                height: 20,
                placeholder: String::from("Please input"),
            }),
            Box::new(SelectBox {
                width: 100,
                height: 20,
                placeholder: String::from("Please select"),
                options: vec![String::from("Option one")],
            }),
            Box::new(Image {
                width: 200,
                height: 100,
                src: String::from("https://example.com/example.jpg"),
            })
        ]
    };
    screen.run();
}

