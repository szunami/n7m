use std::collections::HashSet;

fn from_full_word(full_word: &String) -> Result<String, ()> {
    if full_word.len() < 3 {
        return Err(());
    }
    match full_word.chars().next() {
        Some(first) => match full_word.chars().last() {
            Some(last) => {
                let n = full_word.len() - 2;
                return Ok(std::format!("{}{}{}", first, n, last));
            }
            None => return Err(()),
        },
        None => return Err(()),
    }
}

fn to_full_word<'a>(numeronym: &String, words: HashSet<String>) -> Result<HashSet<String>, ()> {
    // parse out a, n, b
    if numeronym.len() < 3 {
        return Err(());
    }

    let mut results = HashSet::new();

    words.iter().for_each(|word| match from_full_word(word) {
        Ok(candidate_numeronym) => {
            if candidate_numeronym == *numeronym {
                results.insert(word.clone());
            }
        }
        Err(_) => {}
    });

    Ok(results)
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

        let kubernetes = String::from("kubernetes");
        let actual = from_full_word(&kubernetes);
        let expected: Result<String, ()> = Ok(String::from("k8s"));
        assert_eq!(actual, expected);

        let internationalization = String::from("internationalization");
        let actual = from_full_word(&internationalization);
        let expected: Result<String, ()> = Ok(String::from("i18n"));
        assert_eq!(actual, expected);

        let what_the_fuck = String::from("whatthefuck");
        let actual = from_full_word(&what_the_fuck);
        let expected: Result<String, ()> = Ok(String::from("w9k"));
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

    #[test]
    fn to_full_word_works() {
        let mut words: HashSet<String> = HashSet::new();
        words.insert(String::from("kubernetes"));
        words.insert(String::from("asdf"));

        let actual = to_full_word(&String::from("k8s"), words); 

        let mut expected = HashSet::new();
        expected.insert(String::from("kubernetes"));
        assert_eq!(actual.unwrap(), expected);
    }
}
