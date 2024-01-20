pub mod select_box {
    use crate::screen::screen::Draw;

    pub struct SelectBox {
        width: u32,
        height: u32,
        placeholder: String,
        options: Vec<String>,
    }
    impl Draw for SelectBox {
        fn draw(&self) {
            // ...
        }
    }
}

