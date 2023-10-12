use mcve::TreeKey;

pub fn main() -> () {
    <[(); 0] as TreeKey>::metadata();
}
