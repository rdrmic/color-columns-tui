use std::{
    io::Write,
    path::{Path, PathBuf},
};

use crate::{blocks::num_matches_unpacking, errors};

pub struct Scoring {
    level: u16,
    score: u32,
    max_combo: u16,
    highscore: u32,
    accumulated_points: u16,
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

    pub fn add(&mut self, bit_packed_points: u64) {
        let all_matches_points = num_matches_unpacking::unpack_matches_points(bit_packed_points);

        let mut calculated_points_per_direction = [0; 4];
        for direction_points in all_matches_points.into_iter().enumerate() {
            let points = direction_points.1.into_iter().filter(|p| *p > 0).map(u16::from).product::<u16>();
            calculated_points_per_direction[direction_points.0] = if points == 1 { 0 } else { points };
        }

        let cascade_multiplier = self.calculate_cascade_multiplier();

        let calculated_points = calculated_points_per_direction.into_iter().filter(|p| *p > 0).product::<u16>() * cascade_multiplier;
        self.accumulated_points += calculated_points;

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

    pub const fn level(&self) -> u16 {
        self.level
    }

    // TODO
    // pub const fn points(&self) -> u16 {
    //     self.accumulated_points
    // }

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

    const fn calculate_cascade_multiplier(&mut self) -> u16 {
        // *1 *3 *4 *5 etc.
        self.cascade_count += 1;
        if self.cascade_count == 1 {
            return 1;
        }
        1 + self.cascade_count as u16
    }

    fn add_accumulated_points(&mut self) {
        self.score += u32::from(self.accumulated_points);

        if self.accumulated_points > self.max_combo {
            self.max_combo = self.accumulated_points;
        }
    }

    const fn update_highscore(&mut self) {
        if self.score > self.highscore {
            self.highscore = self.score;
        }
    }

    const fn calculate_level(score: u32) -> u16 {
        match score {
            0..50 => 1,
            50..150 => 2,
            150..300 => 3,
            300..500 => 4,
            // For scores 500 and above, every 250 points is a new level
            _ => 5 + ((score - 500) / 250) as u16,
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
            Ok(contents) => Ok(contents.trim().parse::<u32>()?),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(0),
            Err(err) => Err(err.into()),
        }
    }

    #[allow(clippy::single_option_map)]
    fn get_highscore_file_path(app_data_dir_path: Option<&Path>) -> Option<PathBuf> {
        app_data_dir_path.map(|path| path.join(Self::HIGHSCORE_FILE_NAME))
    }
}
