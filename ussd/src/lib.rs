use std::{result, vec};

use thiserror::Error;

const MAX_LENGTH: usize = 182;

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
    #[error("done *ucked up")]
    InvalidTermination,
}

enum GroupParseResult<'a> {
    Valid(UssdGroup<'a>),
    End(UssdGroup<'a>),
}

struct UssdGroup<'a> {
    content: &'a [u8],
}

impl UssdGroup<'_> {
    fn parse(_ussd_group: &[u8]) -> Result<GroupParseResult, GroupParseError> {
        let mut ussd_group_iter = _ussd_group.into_iter();
        let mut next_group_part = ussd_group_iter.next();
        let mut terminated = false;

        while next_group_part.is_some() {
            let value = next_group_part.unwrap();
            if !(48..57).contains(value) {
                return Err(GroupParseError::Unknown);
            } else if value == &32 {
                terminated = true
            }
            next_group_part = ussd_group_iter.next();
            if terminated && next_group_part.is_some() {
                return Err(GroupParseError::InvalidTermination);
            }
        }
        if !terminated {
            return Err(GroupParseError::Unknown);
        }
        let result = match terminated {
            true => GroupParseResult::End(UssdGroup {
                content: _ussd_group,
            }),
            false => GroupParseResult::Valid(UssdGroup {
                content: _ussd_group,
            }),
        };
        return Ok(result);
    }
}

pub fn parse(mut _data: &[u8]) -> Result<Vec<Vec<&u8>>, Error> {
    if _data.len() >= MAX_LENGTH {
        return Err(Error::ToLong {
            over: _data.len() - MAX_LENGTH,
            max_allowed: MAX_LENGTH,
        });
    }
    let is_invalid_start = !_data.starts_with(&[42]) || !_data.starts_with(&[32]);
    if is_invalid_start {
        return Err(Error::InvalidStart { first: 42 });
    }

    let mut ussd_groups_iter = _data.split(|x: &u8| x == &42).map(UssdGroup::parse);

    let mut ussd_terminated = false;
    let mut result: Vec<Vec<&u8>> = Vec::new();
    while let Some(parsed_group_result) = ussd_groups_iter.next() {
        let parsed_group = match parsed_group_result {
            Ok(GroupParseResult::End(r)) => {
                ussd_terminated = true;
                r
            }
            Err(GroupParseError::InvalidTermination) => return Err(Error::InvalidTermination {}),
            Err(GroupParseError::Unknown) => return Err(Error::Unknown {}),
            Ok(GroupParseResult::Valid(r)) => r,
        };
        let group = Vec::from_iter(parsed_group.content);
        result.push(group)
    }
    if !ussd_terminated {
        return Err(Error::InvalidTermination {});
    }
    return Ok(result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let data: &[u8] = b"*";
        let _result = parse(&data[..]).expect("Failed parsing ussd code");
        // assert_eq!(result, 4);
    }

    #[test]
    fn testing() {
        let data: &[u8] = b"**11*22*33#";
        let split: Vec<&[u8]> = data.split(|x: &u8| x == &42 || x == &32).collect();
        print!("{:?}", split);
        let _result = parse(&data[..]).expect("Failed parsing ussd code");
        // assert_eq!(result, 4);
    }
}
