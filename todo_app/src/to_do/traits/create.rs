pub trait Create {
    fn create(&self, title: &str) {
        println!("{} is being to created.", title);
    }
}