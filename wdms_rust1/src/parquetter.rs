use std::fs::File;
use std::io::{BufReader, BufWriter};
use polars::prelude::*;

pub fn try_polars() {
    let df = df!("Fruit" => &["Apple", "Apple", "Pear"],
        "Color" => &[1u16, 4u16, 98u16]).expect("");
    let q = df.column("Color").expect("").quantile_as_series(0.9, QuantileInterpolOptions::default()).expect("");
    println!("{:?}", df);
    println!("{:?}", q);
}

pub fn write_parquet() {
    let mut df = df!("Fruit" => &["Apple", "Apple", "Pear"],
        "Color" => &[1u16, 4u16, 98u16]).expect("");
    let f = File::create("j.parquet").expect("Unable to create the file j.parquet!");
    let mut bfw = BufWriter::new(f);
    let pw = ParquetWriter::new(bfw).with_compression(ParquetCompression::Snappy);
    pw.finish(&mut df);
}

pub fn read_parquet() {
    let f = std::fs::File::open("df_20.parquet").unwrap();
    let mut bfr = BufReader::new(f);
    let df = ParquetReader::new(bfr).finish().unwrap();
    println!("{:?}", df);
}

#[cfg(test)]
mod test {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_foo2() {

    }
}