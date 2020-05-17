//! A RollResult is the object returned by Roll::execute(), including a copy of the original roll
use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct RollResult {
    pub roll: Roll,
    pub base: isize,
}

impl Default for RollResult {
    fn default() -> Self {
        Self {
            roll: Roll::default(),
            base: 1,
        }
    }
}

impl RollResult {
    pub fn new(roll: Roll, base: isize) -> Self {
        Self { roll, base }
    }
    pub fn get_modifier(&self) -> isize {
        self.roll.get_modifier()
    }
    pub fn total(&self) -> isize {
        self.base + self.roll.get_modifier()
    }
}

impl fmt::Display for RollResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let sign = if self.get_modifier() >= 0 { "+" } else { "" };
        write!(
            f,
            "{} ({}{}{})",
            self.total(),
            self.base,
            sign,
            self.roll.get_modifier()
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_default_result() {
        let result = Roll::from_str("1d6").unwrap().execute();
        assert_eq!(result.roll, Roll::default());
    }
    #[test]
    fn test_display_result_pos_modifer() {
        let result = Roll::from_str("1d1+1").unwrap().execute().to_string();
        assert_eq!(&result, "2 (1+1)");
    }
    #[test]
    fn test_display_result_neg_modifer() {
        let result = Roll::from_str("1d1-1").unwrap().execute().to_string();
        assert_eq!(&result, "0 (1-1)");
    }
}
