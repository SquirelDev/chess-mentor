use std::fs::File;
use std::path::Path;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use arrow::array::StringArray;
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Opening {
    pub eco: String,
    pub name: String,
    pub pgn: String,
    pub uci: String,
    pub epd: String,
}

pub fn get_all_openings() -> Result<Vec<Opening>, String> {
    let path = Path::new("../data/train-00000-of-00001.parquet");
    let file = match File::open(&path) {
        Ok(file) => file,
        Err(e) => return Err(format!("Failed to open parquet file: {}", e)),
    };

    let builder = ParquetRecordBatchReaderBuilder::try_new(file).map_err(|e| e.to_string())?;
    let mut reader = builder.build().map_err(|e| e.to_string())?;

    let mut openings = Vec::new();

    while let Some(record_batch) = reader.next() {
        let record_batch = record_batch.map_err(|e| e.to_string())?;
        let eco_array = record_batch
            .column_by_name("eco")
            .unwrap()
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();
        let name_array = record_batch
            .column_by_name("name")
            .unwrap()
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();
        let pgn_array = record_batch
            .column_by_name("pgn")
            .unwrap()
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();
        let uci_array = record_batch
            .column_by_name("uci")
            .unwrap()
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();
        let epd_array = record_batch
            .column_by_name("epd")
            .unwrap()
            .as_any()
            .downcast_ref::<StringArray>()
            .unwrap();

        for i in 0..record_batch.num_rows() {
            openings.push(Opening {
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
