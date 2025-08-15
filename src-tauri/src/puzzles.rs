use std::fs::File;
use std::path::Path;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use arrow::array::{StringArray, Int32Array};
use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Puzzle {
    #[serde(rename = "PuzzleId")]
    pub puzzle_id: String,
    #[serde(rename = "FEN")]
    pub fen: String,
    #[serde(rename = "Moves")]
    pub moves: String,
    #[serde(rename = "Rating")]
    pub rating: i32,
    #[serde(rename = "RatingDeviation")]
    pub rating_deviation: i32,
    #[serde(rename = "Popularity")]
    pub popularity: i32,
    #[serde(rename = "NbPlays")]
    pub nb_plays: i32,
    #[serde(rename = "Themes")]
    pub themes: String,
    #[serde(rename = "GameUrl")]
    pub game_url: String,
}

pub fn get_random_puzzle(level: String) -> Result<Puzzle, String> {
    let (min_rating, max_rating) = match level.as_str() {
        "Easy" => (800, 1200),
        "Medium" => (1201, 1600),
        "Hard" => (1601, 2000),
        _ => (800, 2000),
    };

    let path = Path::new("../data/train-00000-of-00003.parquet");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => return Err(format!("Failed to open parquet file: {}", e)),
    };

    let builder = ParquetRecordBatchReaderBuilder::try_new(file).map_err(|e| e.to_string())?;
    let mut reader = builder.build().map_err(|e| e.to_string())?;

    let mut puzzles = Vec::new();

    while let Some(record_batch) = reader.next() {
        let record_batch = record_batch.map_err(|e| e.to_string())?;
        let puzzle_id_array = record_batch
            .column_by_name("PuzzleId")
            .ok_or("Column 'PuzzleId' not found.".to_string())?
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or("Failed to downcast 'PuzzleId' to StringArray.".to_string())?;
        let fen_array = record_batch
            .column_by_name("FEN")
            .ok_or("Column 'FEN' not found.".to_string())?
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or("Failed to downcast 'FEN' to StringArray.".to_string())?;
        let moves_array = record_batch
            .column_by_name("Moves")
            .ok_or("Column 'Moves' not found.".to_string())?
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or("Failed to downcast 'Moves' to StringArray.".to_string())?;
        let rating_array = record_batch
            .column_by_name("Rating")
            .ok_or("Column 'Rating' not found.".to_string())?
            .as_any()
            .downcast_ref::<Int32Array>()
            .ok_or("Failed to downcast 'Rating' to Int32Array.".to_string())?;
        let rating_deviation_array = record_batch
            .column_by_name("RatingDeviation")
            .ok_or("Column 'RatingDeviation' not found.".to_string())?
            .as_any()
            .downcast_ref::<Int32Array>()
            .ok_or("Failed to downcast 'RatingDeviation' to Int32Array.".to_string())?;
        let popularity_array = record_batch
            .column_by_name("Popularity")
            .ok_or("Column 'Popularity' not found.".to_string())?
            .as_any()
            .downcast_ref::<Int32Array>()
            .ok_or("Failed to downcast 'Popularity' to Int32Array.".to_string())?;
        let nb_plays_array = record_batch
            .column_by_name("NbPlays")
            .ok_or("Column 'NbPlays' not found.".to_string())?
            .as_any()
            .downcast_ref::<Int32Array>()
            .ok_or("Failed to downcast 'NbPlays' to Int32Array.".to_string())?;
        let themes_array = record_batch
            .column_by_name("Themes")
            .ok_or("Column 'Themes' not found.".to_string())?
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or("Failed to downcast 'Themes' to StringArray.".to_string())?;
        let game_url_array = record_batch
            .column_by_name("GameUrl")
            .ok_or("Column 'GameUrl' not found.".to_string())?
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or("Failed to downcast 'GameUrl' to StringArray.".to_string())?;

        for i in 0..record_batch.num_rows() {
            let rating = rating_array.value(i);
            if rating >= min_rating && rating <= max_rating {
                puzzles.push(Puzzle {
                    puzzle_id: puzzle_id_array.value(i).to_string(),
                    fen: fen_array.value(i).to_string(),
                    moves: moves_array.value(i).to_string(),
                    rating,
                    rating_deviation: rating_deviation_array.value(i),
                    popularity: popularity_array.value(i),
                    nb_plays: nb_plays_array.value(i),
                    themes: themes_array.value(i).to_string(),
                    game_url: game_url_array.value(i).to_string(),
                });
            }
        }
    }

    if puzzles.is_empty() {
        return Err("No puzzles found in the specified rating range.".to_string());
    }

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..puzzles.len());

    Ok(puzzles[random_index].clone())
}
