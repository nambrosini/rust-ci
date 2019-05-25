use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Greeting<'a>(&'a str);

impl<'a> Greeting<'a> {
    pub fn greet(target: &str) -> Greeting {
        Greeting(target)
    }
}

impl<'a> fmt::Display for Greeting<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Greetings, {}!", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn greet_author() {
        let greeter = Greeting::greet("nambrosini");
        assert_eq!(greeter.to_string(), "Greetings, nambrosini!");
    }
}