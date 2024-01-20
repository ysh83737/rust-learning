pub mod button {
    use crate::screen::screen::Draw;

    pub struct Button {
        pub width: u32,
        pub height: u32,
        pub label: String,
    }
    impl Draw for Button {
        fn draw(&self) {
            // ...
        }
    }
}
