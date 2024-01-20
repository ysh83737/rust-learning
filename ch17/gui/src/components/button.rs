pub mod button {
    use crate::screen::screen::Draw;

    pub struct Button {
        width: u32,
        height: u32,
        label: String,
    }
    impl Draw for Button {
        fn draw(&self) {
            // ...
        }
    }
}
