use crate::language_enum;
use failure::bail;

language_enum!([EN]);

impl Language {
    pub fn full_name(&self) -> &'static str {
        match *self {
            Language::EN => "English",
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn init_from_lowercased_string_works() {
        let lang = Language::from_str("en");
        assert!(lang.is_ok());
    }

    #[test]
    fn init_from_uppercased_string_works() {
        let lang = Language::from_str("EN");
        assert!(lang.is_ok());
    }
}
