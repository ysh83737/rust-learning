use ch14_art::kinds::PrimaryColor;
use ch14_art::utils::mix;

fn main() {
  let red = PrimaryColor::Red;
  let yellow = PrimaryColor::Yellow;
  mix(red, yellow);
}