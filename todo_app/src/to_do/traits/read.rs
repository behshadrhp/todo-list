pub trait Read {
    fn read(&self, title: &str) {
        println!("{} Is being fetched.", title);
    }
}