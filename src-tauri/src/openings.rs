use std::fs::File;
use std::path::Path;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use arrow::array::StringArray;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Opening {
    #[serde(rename = "eco-volume")]
    pub eco_volume: String,
    pub eco: String,
    pub name: String,
    pub pgn: String,
    pub uci: String,
    pub epd: String,
}

pub fn get_all_openings() -> Result<Vec<Opening>, String> {
    let path = Path::new("../data/openings.parquet");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => return Err(format!("Failed to open parquet file: {}", e)),
    };

    let builder = ParquetRecordBatchReaderBuilder::try_new(file).map_err(|e| e.to_string())?;
    let mut reader = builder.build().map_err(|e| e.to_string())?;

    let mut openings = Vec::new();

    while let Some(record_batch) = reader.next() {
        let record_batch = record_batch.map_err(|e| e.to_string())?;
        let eco_volume_array = record_batch
            .column_by_name("eco-volume")
            .ok_or("Column 'eco-volume' not found.".to_string())?
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or("Failed to downcast 'eco-volume' to StringArray.".to_string())?;
        let eco_array = record_batch
            .column_by_name("eco")
            .ok_or("Column 'eco' not found.".to_string())?
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or("Failed to downcast 'eco' to StringArray.".to_string())?;
        let name_array = record_batch
            .column_by_name("name")
            .ok_or("Column 'name' not found.".to_string())?
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or("Failed to downcast 'name' to StringArray.".to_string())?;
        let pgn_array = record_batch
            .column_by_name("pgn")
            .ok_or("Column 'pgn' not found.".to_string())?
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or("Failed to downcast 'pgn' to StringArray.".to_string())?;
        let uci_array = record_batch
            .column_by_name("uci")
            .ok_or("Column 'uci' not found.".to_string())?
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or("Failed to downcast 'uci' to StringArray.".to_string())?;
        let epd_array = record_batch
            .column_by_name("epd")
            .ok_or("Column 'epd' not found.".to_string())?
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or("Failed to downcast 'epd' to StringArray.".to_string())?;

        for i in 0..record_batch.num_rows() {
            openings.push(Opening {
                eco_volume: eco_volume_array.value(i).to_string(),
                eco: eco_array.value(i).to_string(),
                name: name_array.value(i).to_string(),
                pgn: pgn_array.value(i).to_string(),
                uci: uci_array.value(i).to_string(),
                epd: epd_array.value(i).to_string(),
            });
        }
    }

    Ok(openings)
}
