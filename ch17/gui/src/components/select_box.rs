pub mod select_box {
    use crate::screen::screen::Draw;

    pub struct SelectBox {
        pub width: u32,
        pub height: u32,
        pub placeholder: String,
        pub options: Vec<String>,
    }
    impl Draw for SelectBox {
        fn draw(&self) {
            // ...
        }
    }
}

