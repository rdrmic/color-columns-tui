use std::{
    io::Write,
    path::{Path, PathBuf},
};

use crate::errors;

pub struct Scoring {
    level: u32,
    score: u32,
    max_combo: u32,
    highscore: u32,
    accumulated_points: u32,
    cascade_count: u8,
}

impl Scoring {
    const HIGHSCORE_FILE_NAME: &str = "hs";

    #[rustfmt::skip]
    pub fn new(app_data_dir_path: Option<&Path>) -> Result<Self, errors::Error> {
        Ok(
            Self {
                level: 1,
                score: 0,
                max_combo: 0,
                highscore: Self::read_highscore_from_file(app_data_dir_path)?,
                accumulated_points: 0,
                cascade_count: 0,
            }
        )
    }

    pub const fn add(&mut self, match_points: u32) {
        let calculated_points = match_points.saturating_mul(self.calculate_cascade_multiplier());
        self.accumulated_points = self.accumulated_points.saturating_add(calculated_points);

        self.add_accumulated_points();

        self.update_highscore();
    }

    pub const fn is_level_increased(&mut self) -> bool {
        let calculated_level = Self::calculate_level(self.score);
        if calculated_level > self.level {
            self.level = calculated_level;
            true
        } else {
            false
        }
    }

    pub const fn level(&self) -> u32 {
        self.level
    }

    // TODO
    // pub const fn accumulated_points(&self) -> u32 {
    //     self.accumulated_points
    // }

    pub const fn score(&self) -> u32 {
        self.score
    }

    pub const fn max_combo(&self) -> u32 {
        self.max_combo
    }

    pub const fn highscore(&self) -> u32 {
        self.highscore
    }

    pub const fn break_cascade_sequence(&mut self) {
        self.accumulated_points = 0;
        self.cascade_count = 0;
    }

    const fn calculate_cascade_multiplier(&mut self) -> u32 {
        // *1 *3 *4 *5 etc.
        self.cascade_count += 1;
        if self.cascade_count == 1 {
            return 1;
        }
        1 + self.cascade_count as u32
    }

    const fn add_accumulated_points(&mut self) {
        self.score = self.score.saturating_add(self.accumulated_points);

        if self.accumulated_points > self.max_combo {
            self.max_combo = self.accumulated_points;
        }
    }

    const fn update_highscore(&mut self) {
        if self.score > self.highscore {
            self.highscore = self.score;
        }
    }

    const fn calculate_level(score: u32) -> u32 {
        match score {
            0..50 => 1,
            50..150 => 2,
            150..300 => 3,
            300..500 => 4,
            // For scores 500 and above, every 250 points is a new level
            _ => 5 + (score - 500) / 250,
        }
    }

    // =============================================================================
    // Highscore: reading from and writing to file
    // =============================================================================
    pub fn write_highscore_to_file(&self, app_data_dir_path: Option<&Path>) -> Result<(), errors::Error> {
        if let Some(file_path) = Self::get_highscore_file_path(app_data_dir_path) {
            let mut file = std::fs::File::create(file_path)?;
            write!(file, "{}", self.highscore())?;
        }
        Ok(())
    }

    fn read_highscore_from_file(app_data_dir_path: Option<&Path>) -> Result<u32, errors::Error> {
        let Some(file_path) = Self::get_highscore_file_path(app_data_dir_path) else {
            return Ok(0);
        };

        match std::fs::read_to_string(&file_path) {
            Ok(contents) => Self::parse_highscore(contents.trim()),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(0),
            Err(err) => Err(err.into()),
        }
    }

    fn parse_highscore(input: &str) -> Result<u32, errors::Error> {
        let bytes = input.as_bytes();
        let Some(&first) = bytes.first() else {
            return Err(errors::Error::ParseIntEmpty);
        };

        let mut index = usize::from(first == b'+');
        if index == bytes.len() {
            return Err(errors::Error::ParseIntInvalidDigit);
        }

        let mut value = 0_u32;
        while index < bytes.len() {
            let digit = bytes[index].wrapping_sub(b'0');
            if digit > 9 {
                return Err(errors::Error::ParseIntInvalidDigit);
            }

            let Some(next_value) = value.checked_mul(10).and_then(|value| value.checked_add(u32::from(digit))) else {
                return Err(errors::Error::ParseIntPosOverflow);
            };
            value = next_value;
            index += 1;
        }

        Ok(value)
    }

    #[allow(clippy::single_option_map)]
    fn get_highscore_file_path(app_data_dir_path: Option<&Path>) -> Option<PathBuf> {
        app_data_dir_path.map(|path| path.join(Self::HIGHSCORE_FILE_NAME))
    }
}

// =============================================================================
// TESTS
// =============================================================================
#[cfg(test)]
mod tests {
    use super::Scoring;

    #[test]
    fn applies_match_and_cascade_products() {
        let mut scoring = Scoring::new(None).expect("scoring without persistence should initialize");

        scoring.add(3);
        assert_eq!(scoring.score(), 3);

        scoring.add(3);
        assert_eq!(scoring.score(), 15);
    }

    #[test]
    fn saturates_points_at_the_score_limit() {
        let mut scoring = Scoring::new(None).expect("scoring without persistence should initialize");

        scoring.add(u32::MAX);
        scoring.add(u32::MAX);

        assert_eq!(scoring.score(), u32::MAX);
        assert_eq!(scoring.max_combo(), u32::MAX);
    }

    #[test]
    fn compact_highscore_parser_matches_u32_parsing() {
        let cases = [
            "",
            "+",
            "-",
            "0",
            "+0",
            "0001",
            "42",
            "4294967295",
            "4294967296",
            "-1",
            "1a",
            "１２",
            "00000000000000000000000000000000000000000000000001",
            "99999999999999999999999999999999999999999999999999",
        ];

        for input in cases {
            let expected = input.parse::<u32>().map_err(|error| error.to_string());
            let actual = Scoring::parse_highscore(input).map_err(|error| error.to_string());
            assert_eq!(actual, expected, "input: {input:?}");
        }
    }
}
