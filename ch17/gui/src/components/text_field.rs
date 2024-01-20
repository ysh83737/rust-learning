pub mod text_filed {
    use crate::screen::screen::Draw;

    pub struct TextField {
        pub width: u32,
        pub height: u32,
        pub placeholder: String,
    }
    impl Draw for TextField {
        fn draw(&self) {
            println!("Draw TextField: width={}, height={}, placeholder={}", self.width, self.height, self.placeholder);
        }
    }
}
