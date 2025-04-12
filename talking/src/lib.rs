pub fn talking(text: &str) -> &str {
    let trimmed = text.trim();

    if trimmed.is_empty() {
        return "Just say something!";
    }

    let is_yelling = trimmed
        .chars()
        .filter(|c| c.is_alphabetic()) // Only consider alphabetic characters for yelling
        .all(|c| c.is_uppercase())
        && trimmed.chars().any(|c| c.is_alphabetic()); // Ensure there's at least one letter

    let is_question = trimmed.ends_with('?');

    if is_question && is_yelling {
        "Quiet, I am thinking!"
    } else if is_question {
        "Sure."
    } else if is_yelling {
        "There is no need to yell, calm down!"
    } else {
        "Interesting"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shouting() {
        assert_eq!(talking("WATCH OUT!"), "There is no need to yell, calm down!");
    }

    #[test]
    fn asking() {
        assert_eq!(talking("Does this cryogenic sleep chamber work?"), "Sure.");
    }

    #[test]
    fn asking_yelling() {
        assert_eq!(talking("WHAT'S GOING ON?"), "Quiet, I am thinking!");
    }

    #[test]
    fn nothing() {
        assert_eq!(talking(""), "Just say something!");
    }

    #[test]
    fn whitespace() {
        assert_eq!(talking("   "), "Just say something!");
    }

    #[test]
    fn talking_normally() {
        assert_eq!(talking("Oh, hi"), "Interesting");
    }

    #[test]
    fn non_letters() {
        assert_eq!(talking("1, 2, 3"), "Interesting");
    }

    #[test]
    fn shouting_with_numbers() {
        assert_eq!(talking("123 WATCH OUT!"), "There is no need to yell, calm down!");
    }
}