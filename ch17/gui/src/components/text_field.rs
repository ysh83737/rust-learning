pub mod text_filed {
    use crate::screen::screen::Draw;

    pub struct TextField {
        width: u32,
        height: u32,
        placeholder: String,
    }
    impl Draw for TextField {
        fn draw(&self) {
            // ...
        }
    }
}
