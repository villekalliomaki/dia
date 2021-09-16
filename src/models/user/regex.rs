use regex::Regex;

lazy_static! {
    pub static ref USERNAME: Regex = Regex::new(r"^[A-Za-z0-9_-]{4,20}$").unwrap();
    pub static ref PASSWORD: Regex = Regex::new(r"^.{20,50}$").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn username_valid() {
        assert!(USERNAME.is_match("valid_username"));

        assert!(USERNAME.is_match("withNumbers123"));
    }

    #[test]
    fn username_invalid() {
        assert!(!USERNAME.is_match("with space"));

        assert!(!USERNAME.is_match("ääkkösillä"));
    }

    #[test]
    fn password_valid() {
        assert!(PASSWORD.is_match("WDDsKtbvkZK3UjbYwboiV72cVXQ2c8"));
    }

    #[test]
    fn password_invalid() {
        assert!(!PASSWORD.is_match("too short"));

        assert!(!PASSWORD.is_match("too long Vo3Qt32cahHt6aTa3urxAwM32L2JrSVo3Qt32cahHt6aTa3urxAwM32L2JrSVo3Qt32cahHt6aTa3urxAwM32L2JrS"));
    }
}
