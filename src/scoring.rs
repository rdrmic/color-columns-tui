use std::{
    io::Write,
    path::{Path, PathBuf},
};

use crate::errors;

pub struct Scoring {
    score: u32,
    max_combo: u16,
    highscore: u32,
    accumulated_points: u16,
    cascade_count: u8,
}

impl Scoring {
    const HIGHSCORE_FILE_NAME: &str = "hs";

    #[rustfmt::skip]
    pub fn new(app_state_dir_path: Option<&Path>) -> Result<Self, errors::Error> {
        Ok(
            Self {
                score: 0,
                max_combo: 0,
                highscore: Self::read_highscore_from_file(app_state_dir_path)?,
                accumulated_points: 0,
                cascade_count: 0,
            }
        )
    }

    pub fn add(&mut self, points: [[u8; 4]; 4]) {
        let mut calculated_points_per_direction = [0; 4];
        for direction_points in points.into_iter().enumerate() {
            let points = direction_points.1.into_iter().filter(|p| *p > 0).map(u16::from).product::<u16>();
            calculated_points_per_direction[direction_points.0] = if points == 1 { 0 } else { points };
        }

        let cascade_multiplier = self.calculate_cascade_multiplier();

        let calculated_points = calculated_points_per_direction.into_iter().filter(|p| *p > 0).product::<u16>() * cascade_multiplier;
        self.accumulated_points += calculated_points;
    }

    pub fn count_in_accumulated_points(&mut self) {
        self.score += u32::from(self.accumulated_points);

        if self.accumulated_points > self.max_combo {
            self.max_combo = self.accumulated_points;
        }

        if self.score > self.highscore {
            self.highscore = self.score;
        }
    }

    pub const fn score(&self) -> u32 {
        self.score
    }

    pub const fn max_combo(&self) -> u16 {
        self.max_combo
    }

    pub const fn highscore(&self) -> u32 {
        self.highscore
    }

    pub const fn break_cascade_sequence(&mut self) {
        self.accumulated_points = 0;
        self.cascade_count = 0;
    }

    pub fn write_highscore_to_file(&self, app_state_dir_path: Option<&Path>) -> Result<(), errors::Error> {
        let file_path = Self::get_highscore_filepath(app_state_dir_path)?;

        let mut file = std::fs::File::create(file_path)?;
        write!(file, "{}", self.highscore())?;
        Ok(())
    }

    fn read_highscore_from_file(app_state_dir_path: Option<&Path>) -> Result<u32, errors::Error> {
        let filepath = Self::get_highscore_filepath(app_state_dir_path)?;

        let contents = match std::fs::read_to_string(&filepath) {
            Ok(contents) => contents,
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => return Ok(0),
            Err(err) => return Err(err.into()),
        };

        let highscore = contents.trim().parse::<u32>()?;
        Ok(highscore)
    }

    fn get_highscore_filepath(app_state_dir_path: Option<&Path>) -> Result<PathBuf, errors::Error> {
        let path = app_state_dir_path.ok_or_else(|| std::io::Error::new(std::io::ErrorKind::NotFound, "App state directory path is missing"))?;
        Ok(path.join(Self::HIGHSCORE_FILE_NAME))
    }

    const fn calculate_cascade_multiplier(&mut self) -> u16 {
        // *1 *3 *4 *5 etc.
        self.cascade_count += 1;
        if self.cascade_count == 1 {
            return 1;
        }
        1 + self.cascade_count as u16
    }
}
