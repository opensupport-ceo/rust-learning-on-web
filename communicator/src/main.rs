/*

reference:=> https://rinthel.github.io/rust-lang-book-ko/ch07-02-controlling-visibility-with-pub.html

*/

extern crate communicator;

fn main() {
    communicator::client::connect();
}
