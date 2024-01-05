use ch11_03_adder;

mod common;

#[test]
fn two_and_two_is_four() {
  common::setup();
  let result = ch11_03_adder::add(2, 2);
  assert_eq!(result, 4);
}