use thiserror::Error;

const MAX_LENGTH: usize = 182;

#[derive(Debug)]
pub struct Ussd {}

#[derive(Debug, Error)]
pub enum Error {
    #[error("USSD candidate was {over} bytes over allowed {MAX_LENGTH}")]
    ToLong { over: usize, max_allowed: usize },
    #[error("done *ucked up")]
    InvalidTermination {},
    #[error("done *ucked up")]
    InvalidStart { first: u8 },
    #[error("done *ucked up")]
    Unknown,
}
#[derive(Debug, Error)]

enum GroupParseError {
    #[error("done *ucked up")]
    Unknown,
}

enum GroupParseResult<'a> {
    Valid(UssdGroup<'a>),
    End(UssdGroup<'a>),
}

struct UssdGroup<'a> {
    content: &'a [u8],
}

impl UssdGroup<'_> {
    fn parse(_x: &[u8]) -> Result<GroupParseResult, GroupParseError> {
        let mut next = _x.into_iter().next();

       while next.is_some() {
           
       }

        return Err(GroupParseError::Unknown);
    }
}

impl Ussd {
    pub fn new() -> Self {
        todo!()
    }

    pub fn parse(mut _data: &[u8]) -> Result<Self, Error> {
        let is_valid: bool = false;

        if _data.len() >= MAX_LENGTH {
            return Err(Error::ToLong {
                over: _data.len() - MAX_LENGTH,
                max_allowed: MAX_LENGTH,
            });
        }
        let split: Vec<UssdGroup> = _data
            .split(|x: &u8| x == &42)
            .map(UssdGroup::parse)
            .map(|x| x.unwrap())
            .collect();

        print!("AAAAAAAA{:?}", _data.first());
        if !_data.starts_with(&[42]) || !_data.starts_with(&[32]) {
            return Err(Error::InvalidStart { first: 42 });
        }
        if !is_valid {
            return Err(Error::Unknown);
        } else {
            return Ok(Ussd::new());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data: &[u8] = b"*";
        let _result = Ussd::parse(&data[..]).expect("Failed parsing ussd code");
        // assert_eq!(result, 4);
    }

    #[test]
    fn testing() {
        let data: &[u8] = b"**11*22*33#";
        let split: Vec<&[u8]> = data.split(|x: &u8| x == &42 || x == &32).collect();
        print!("{:?}", split);
        let _result = Ussd::parse(&data[..]).expect("Failed parsing ussd code");
        // assert_eq!(result, 4);
    }
}
