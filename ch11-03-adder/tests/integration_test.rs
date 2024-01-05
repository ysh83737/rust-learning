use ch11_03_adder;

#[test]
fn two_and_two_is_four() {
  let result = ch11_03_adder::add(2, 2);
  assert_eq!(result, 4);
}