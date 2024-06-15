use thiserror::Error;

#[derive(Debug)]
pub struct Ussd {}

#[derive(Debug, Error)]
pub enum Error {}

impl Ussd {
    pub fn new() -> Self {
        todo!()
    }

    pub fn parse(_data: &[u8]) -> Result<Self, Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data = b"**002*number*BS*T#";
        let _result = Ussd::parse(&data[..]).expect("Failed parsing ussd code");
        // assert_eq!(result, 4);
    }
}
