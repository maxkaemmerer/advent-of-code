pub type Token<'a> = (&'a str, &'a str);

pub fn parse_token<'a>(
    string: &'a str,
    token_name: &'a str,
    start_delimiter: &'a str,
    end_delimiter: char,
) -> Option<Token<'a>> {
    let mut search_string = token_name.to_owned();
    search_string.push_str(start_delimiter);
    let index = string.find(search_string.as_str());
    if let Some(found_index) = index {
        let (_, rest_containing_value) = string.split_at(found_index + search_string.len());
        let list: Vec<&str> = rest_containing_value.split(end_delimiter).collect();
        return if let [value, ..] = list[..] {
            Some((token_name, value))
        } else {
            None
        };
    }

    None
}

pub fn parse_token_before<'a>(
    string: &'a str,
    token_name: &'a str,
    start_delimiter: &'a str,
    end_delimiter: char,
) -> Option<Token<'a>> {
    let mut search_string = end_delimiter.to_string().to_owned();
    search_string.push_str(token_name);
    let index = string.find(search_string.as_str());
    if let Some(found_index) = index {
        let last_value = string.split_at(found_index).0.split(start_delimiter).last();
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
    use crate::tokens::parse_token;
    use crate::tokens::parse_token_before;

    #[test]
    pub fn should_parse_after() {
        assert_eq!(
            Some(("Game", "3")),
            parse_token(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
                "Game",
                " ",
                ':'
            )
        );
    }
    #[test]
    pub fn should_parse_before() {
        assert_eq!(
            Some(("green", "2")),
            parse_token_before("1 red, 2 green, 6 blue", "green", " ", ' ')
        );
        assert_eq!(
            Some(("red", "1")),
            parse_token_before("Game 100: 1 red, 2 green, 6 blue", "red", " ", ' ')
        );
        assert_eq!(
            Some(("blue", "6")),
            parse_token_before("1 red, 2 green, 6 blue", "blue", " ", ' ')
        );
    }
}
