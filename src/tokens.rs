use std::str::FromStr;

pub type Token<'a, T> = (&'a str, T);

pub fn parse_token_value_after<'a, T: FromStr>(
    string: &str,
    token_name: &'a str,
    delimiter_before_value: &str,
    delimiter_after_value: &str,
) -> Option<Token<'a, T>> {
    let mut search_string = token_name.to_owned();
    search_string.push_str(delimiter_before_value);
    let index = string.find(search_string.as_str());
    if let Some(found_index) = index {
        let (_, rest_containing_value) = string.split_at(found_index + search_string.len());
        let list: Vec<&str> = rest_containing_value.split(delimiter_after_value).collect();

        if let [value, ..] = list[..] {
            if let Some(parsed_value) = value.parse::<T>().ok() {
                return Some((token_name, parsed_value));
            }
        }
    }

    None
}

pub fn parse_token_value_before<'a, T: FromStr>(
    string: &str,
    token_name: &'a str,
    delimiter_before_value: &str,
    delimiter_after_value: &str,
) -> Option<Token<'a, T>> {
    let mut search_string = delimiter_after_value.to_string().to_owned();
    search_string.push_str(token_name);
    let index = string.find(search_string.as_str());
    if let Some(found_index) = index {
        let last_value = string
            .split_at(found_index)
            .0
            .split(delimiter_before_value)
            .last()
            .and_then(|value| value.parse::<T>().ok());
        return if let Some(value) = last_value {
            Some((token_name, value))
        } else {
            None
        };
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::tokens::parse_token_value_after;
    use crate::tokens::parse_token_value_before;

    #[test]
    pub fn should_parse_after() {
        assert_eq!(
            Some(("Game", 3)),
            parse_token_value_after(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                "Game",
                " ",
                ":"
            )
        );

        assert_eq!(
            Some(("Game", 3)),
            parse_token_value_after(
                "8 green, Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                "Game",
                " ",
                ":"
            )
        );
    }
    #[test]
    pub fn should_parse_before() {
        assert_eq!(
            Some(("green", 2)),
            parse_token_value_before("1 red, 2 green, 6 blue", "green", " ", " ")
        );
        assert_eq!(
            Some(("red", 1)),
            parse_token_value_before("Game 100: 1 red, 2 green, 6 blue", "red", " ", " ")
        );
        assert_eq!(
            Some(("blue", 6)),
            parse_token_value_before("1 red, 2 green, 6 blue", "blue", " ", " ")
        );
    }
}
