use std::fs::File;
use std::path::Path;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use arrow::array::{StringArray, UInt16Array, Int8Array, Int64Array, ListArray};
use rand::Rng;
use serde::{Serialize, Deserialize};
use once_cell::sync::Lazy;
use std::sync::Mutex;

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
    pub nb_plays: i64,
    #[serde(rename = "Themes")]
    pub themes: String,
    #[serde(rename = "GameUrl")]
    pub game_url: String,
    #[serde(rename = "OpeningTags")]
    pub opening_tags: String,
}

static PUZZLES: Lazy<Mutex<Vec<Puzzle>>> = Lazy::new(|| {
    Mutex::new(load_all_puzzles().expect("Failed to load puzzles."))
});

pub fn initialize_puzzles() {
    // Eagerly initialize the puzzles
    let _ = PUZZLES.lock();
}

fn load_all_puzzles() -> Result<Vec<Puzzle>, String> {
    let puzzle_files = ["puzzles-0.parquet", "puzzles-1.parquet", "puzzles-2.parquet"];
    let mut puzzles = Vec::new();

    for puzzle_file in puzzle_files.iter() {
        let path_str = format!("../data/{}", puzzle_file);
        let path = Path::new(&path_str);
        let file = match File::open(&path) {
            Ok(file) => file,
            Err(e) => return Err(format!("Failed to open parquet file {}: {}", puzzle_file, e)),
        };

        let builder = ParquetRecordBatchReaderBuilder::try_new(file).map_err(|e| e.to_string())?;
        let mut reader = builder.build().map_err(|e| e.to_string())?;

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
                .downcast_ref::<UInt16Array>()
                .ok_or("Failed to downcast 'Rating' to UInt16Array.".to_string())?;
            let rating_deviation_array = record_batch
                .column_by_name("RatingDeviation")
                .ok_or("Column 'RatingDeviation' not found.".to_string())?
                .as_any()
                .downcast_ref::<UInt16Array>()
                .ok_or("Failed to downcast 'RatingDeviation' to UInt16Array.".to_string())?;
            let popularity_array = record_batch
                .column_by_name("Popularity")
                .ok_or("Column 'Popularity' not found.".to_string())?
                .as_any()
                .downcast_ref::<Int8Array>()
                .ok_or("Failed to downcast 'Popularity' to Int8Array.".to_string())?;
            let nb_plays_array = record_batch
                .column_by_name("NbPlays")
                .ok_or("Column 'NbPlays' not found.".to_string())?
                .as_any()
                .downcast_ref::<Int64Array>()
                .ok_or("Failed to downcast 'NbPlays' to Int64Array.".to_string())?;
            let themes_array = record_batch
                .column_by_name("Themes")
                .ok_or("Column 'Themes' not found.".to_string())?
                .as_any()
                .downcast_ref::<ListArray>()
                .ok_or("Failed to downcast 'Themes' to ListArray.".to_string())?;
            let game_url_array = record_batch
                .column_by_name("GameUrl")
                .ok_or("Column 'GameUrl' not found.".to_string())?
                .as_any()
                .downcast_ref::<StringArray>()
                .ok_or("Failed to downcast 'GameUrl' to StringArray.".to_string())?;
            let opening_tags_array = record_batch
                .column_by_name("OpeningTags")
                .ok_or("Column 'OpeningTags' not found.".to_string())?
                .as_any()
                .downcast_ref::<ListArray>()
                .ok_or("Failed to downcast 'OpeningTags' to ListArray.".to_string())?;

            for i in 0..record_batch.num_rows() {
                let themes_list = themes_array.value(i);
                let themes_str_array = themes_list.as_any().downcast_ref::<StringArray>().ok_or("Themes list inner array is not StringArray")?;
                let themes: Vec<String> = themes_str_array.iter().filter_map(|s| s.map(|s| s.to_string())).collect();

                let opening_tags_list = opening_tags_array.value(i);
                let opening_tags_str_array = opening_tags_list.as_any().downcast_ref::<StringArray>().ok_or("OpeningTags list inner array is not StringArray")?;
                let opening_tags: Vec<String> = opening_tags_str_array.iter().filter_map(|s| s.map(|s| s.to_string())).collect();

                puzzles.push(Puzzle {
                    puzzle_id: puzzle_id_array.value(i).to_string(),
                    fen: fen_array.value(i).to_string(),
                    moves: moves_array.value(i).to_string(),
                    rating: rating_array.value(i) as i32,
                    rating_deviation: rating_deviation_array.value(i) as i32,
                    popularity: popularity_array.value(i) as i32,
                    nb_plays: nb_plays_array.value(i),
                    themes: themes.join(", "),
                    game_url: game_url_array.value(i).to_string(),
                    opening_tags: opening_tags.join(", "),
                });
            }
        }
    }
    Ok(puzzles)
}

pub fn get_random_puzzle(level: String) -> Result<Puzzle, String> {
    let (min_rating, max_rating) = match level.as_str() {
        "Easy" => (800, 1200),
        "Medium" => (1201, 1600),
        "Hard" => (1601, 2000),
        _ => (800, 2000),
    };

    let puzzles_lock = PUZZLES.lock().unwrap();
    let filtered_puzzles: Vec<Puzzle> = puzzles_lock
        .iter()
        .filter(|p| p.rating >= min_rating && p.rating <= max_rating)
        .cloned()
        .collect();

    if filtered_puzzles.is_empty() {
        return Err("No puzzles found in the specified rating range.".to_string());
    }

    let mut rng = rand::thread_rng();
    let random_index = rng.gen_range(0..filtered_puzzles.len());
    Ok(filtered_puzzles[random_index].clone())
}
