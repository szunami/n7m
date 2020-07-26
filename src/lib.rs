fn from_full_word(full_word: &String) -> Result<String, ()> {
    if full_word.len() < 3 {
        return Err(())
    }
    match full_word.chars().next() {
        Some(first) => match full_word.chars().last() {
            Some(last) => {

                let n = full_word.len() - 2;
                return Ok(std::format!("{}{}{}", first, n, last))
            }
            None => return Err(()),
        },
        None => return Err(()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normal_cases_work() {
        let asdf = String::from("asdf");
        let actual = from_full_word(&asdf);
        let expected: Result<String, ()> = Ok(String::from("a2f"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn bad_input_works() {
        let a = String::from("a");
        let actual = from_full_word(&a);
        let expected: Result<String, ()> = Err(());
        assert_eq!(actual, expected);

        let empty = String::from("");
        let actual = from_full_word(&empty);
        let expected: Result<String, ()> = Err(());
        assert_eq!(actual, expected);
    }
}
