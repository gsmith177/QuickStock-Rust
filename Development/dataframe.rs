
use polars::prelude::*;
use polars::frame::DataFrame;

fn dataframe_example() -> PolarsResult<()> {
    let df = CsvReader::from_path("items.csv")?
        .infer_schema(None)
        .has_header(true)
        .finish()?;
    println!("{:?}", df);
    Ok(())
}

fn main() {
    dataframe_example().unwrap();
}
