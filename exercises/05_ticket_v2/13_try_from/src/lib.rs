// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

use crate::Status::{Done, InProgress, ToDo};

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(thiserror::Error, Debug, PartialEq)]
#[error("Status can't be parsed {invalid_status}")]
struct ParseStatusError {
    invalid_status: String
}

impl ParseStatusError {
    pub fn new(invalid_status: String) -> Self {
        Self { invalid_status }
    }
}

impl TryFrom<&str> for Status {
    type Error = ParseStatusError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.to_lowercase().eq("todo") {
            Ok(ToDo)
        } else if value.to_lowercase().eq("inprogress") {
            Ok(InProgress)
        } else if value.to_lowercase().eq("done") {
            Ok(Done)
        } else {
            Err(ParseStatusError {invalid_status: "none of the valid statuses".into()})
        }
    }
}

impl TryFrom<String> for Status {
    type Error = ParseStatusError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Status::try_from(value.as_str())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str_error() {
        let status = Status::try_from("pippo");
        assert_eq!(status.err(), Some(ParseStatusError { invalid_status: "none of the valid statuses".into()}));
    }
}
