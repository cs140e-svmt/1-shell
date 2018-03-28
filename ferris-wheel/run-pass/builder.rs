// FIXME: Make me pass! Diff budget: 30 lines.
struct Builder {
    string: Option<String>,
    number: Option<usize>,
}

impl Builder {
    pub fn default() -> Builder {
        Builder { string: None, number: None }
    }
    fn to_string(&self) -> String {
        let mut result = String::new();
        match self.string {
            None => { }
            Some(ref val) => { result.push_str(&val) }
        }
        match self.number {
            None => { }
            Some(val) => { Builder::addSpace(&mut result); result.push_str(&val.to_string()) }
        }
        result
    }
    fn addSpace(val: &mut String) {
        if !val.is_empty(){
            val.push_str(" ");
        }
    }
    fn string<S>(&mut self, val: S) -> &mut Self where S: Into<String> {
        self.string = Some(val.into());
        self
    }
    fn number(&mut self, val: usize) -> &mut Self {
        self.number = Some(val);
        self
    } 
}

// Do not modify this function.
fn main() {
    let empty = Builder::default().to_string();
    assert_eq!(empty, "");

    let just_str = Builder::default().string("hi").to_string();
    assert_eq!(just_str, "hi");

    let just_num = Builder::default().number(254).to_string();
    assert_eq!(just_num, "254");

    let a = Builder::default()
        .string("hello, world!")
        .number(200)
        .to_string();

    assert_eq!(a, "hello, world! 200");

    let b = Builder::default()
        .string("hello, world!")
        .number(200)
        .string("bye now!")
        .to_string();

    assert_eq!(b, "bye now! 200");

    let c = Builder::default()
        .string("heap!".to_owned())
        .to_string();

    assert_eq!(c, "heap!");

    let d = Builder::default()
        .string("hello, world!")
        .string("goodbye")
        .to_string();

    assert_eq!(d, "goodbye");
}
