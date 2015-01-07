struct DB<'a> {
    people: Vec<&'a str>,
}
impl<'a> DB<'a> {
    fn find(&self, search: &str) -> Option<&str> {
        for i in self.people.iter() {
            if *i == search {
                return Some(*i)
            }
        }
        None
    }
}
fn main() {
    let x = DB {
        people: vec!["Jim", "John"]
    };
    println!("{}", x.find("Jim").unwrap());
}
