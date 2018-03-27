// FIXME: Make me pass! Diff budget: 30 lines.

struct Builder {
    string: Option<String>,
    number: Option<usize>,
}

impl Builder {
    fn default() -> Builder {
        Builder {
            string: None,
            number: None
        }
    }

    fn string<S: Into<String>>(&mut self, string: S) -> &mut Self {
        self.string = Some(string.into());
        self
    }

    fn number(&mut self, number: usize) -> &mut Self {
        self.number = Some(number);
        self
    }

    fn to_string(&self) -> String {
        let string_part = self.string.as_ref().map_or_else(|| "", |v| v.as_str());
        let number_part = match self.number {
            None => String::new(),
            Some(ref number) => number.to_string()
        };

        [string_part, number_part.as_str()].join(" ").trim().to_owned()
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
}
