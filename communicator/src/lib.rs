/*

Reference:=> https://rinthel.github.io/rust-lang-book-ko/ch07-01-mod-and-the-filesystem.html

*/
mod client;

mod network;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
