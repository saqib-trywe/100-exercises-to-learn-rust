// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketDescription` type,
//   enforcing that the description is not empty and is not longer than 500 bytes.
//   Implement the traits required to make the tests pass too.

use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub enum DescriptionError {
    DescriptionEmpty,
    DecriptionTooLong,
}

impl Display for DescriptionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DescriptionEmpty => f.write_str("The description cannot be empty"),
            Self::DecriptionTooLong => {
                f.write_str("The description cannot be longer than 500 bytes")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TicketDescription(String);

impl TryFrom<String> for TicketDescription {
    type Error = DescriptionError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.trim() {
            "" => Err(DescriptionError::DescriptionEmpty),
            x if x.len() > 500 => Err(DescriptionError::DecriptionTooLong),
            desc => Ok(TicketDescription(desc.to_string())),
        }
    }
}

impl TryFrom<&str> for TicketDescription {
    type Error = DescriptionError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.trim() {
            "" => Err(DescriptionError::DescriptionEmpty),
            x if x.len() > 500 => Err(DescriptionError::DecriptionTooLong),
            desc => Ok(TicketDescription(desc.to_string())),
        }
    }
}

impl Display for TicketDescription {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let description = TicketDescription::try_from("A description".to_string()).unwrap();
        assert_eq!(description.0, "A description");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketDescription::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The description cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let description = "At vero eos et accusamus et iusto odio dignissimos ducimus qui blanditiis praesentium voluptatum deleniti atque corrupti quos dolores et quas molestias excepturi sint occaecati cupiditate non provident, similique sunt in culpa qui officia deserunt mollitia animi, id est laborum et dolorum fuga. Et harum quidem rerum facilis est et expedita distinctio. Nam libero tempore, cum soluta nobis est eligendi optio cumque nihil impedit quo minus id quod maxime placeat facere possimus, omnis voluptas assumenda est, omnis dolor repellendus. Temporibus autem quibusdam et aut officiis debitis aut rerum necessitatibus saepe eveniet ut et voluptates repudiandae sint et molestiae non recusandae. Itaque earum rerum hic tenetur a sapiente delectus, ut aut reiciendis voluptatibus maiores alias consequatur aut perferendis doloribus asperiores repellat.".to_string();
        let err = TicketDescription::try_from(description).unwrap_err();
        assert_eq!(
            err.to_string(),
            "The description cannot be longer than 500 bytes"
        );
    }

    #[test]
    fn test_try_from_str() {
        let description = TicketDescription::try_from("A description").unwrap();
        assert_eq!(description.0, "A description");
    }
}
