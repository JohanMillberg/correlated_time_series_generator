use chrono::{DateTime, Local, TimeZone};
use csv::Writer;
use ndarray::Array2;
use std::error::Error;

pub fn write_to_csv(file_name: &str, data: &Array2<f64>) -> Result<(), Box<dyn Error>> {
    let today = Local::now();

    let mut writer = Writer::from_path(file_name)?;
    for i in 0..data.nrows() {
        let date = today - chrono::Duration::days(data.nrows() as i64 - i as i64 - 1);
        let date_str = date.format("%Y-%M-%d").to_string();

        let row = data.row(i);
        let mut record = vec![date_str];
        for &value in row.iter() {
            record.push(value.to_string());
        }
        writer.write_record(record)?;
    }
    writer.flush()?;

    Ok(())
}
