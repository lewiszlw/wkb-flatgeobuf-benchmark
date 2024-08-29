use std::io::{BufReader, BufWriter};

use criterion::{criterion_group, criterion_main, Criterion};
use flatgeobuf::{FgbReader, FgbWriter, GeometryType};
use geos::Geom;
use geozero::wkt::WktReader;
use geozero::GeozeroDatasource;

mod data;

async fn parse_fgb(fgb: &[u8]) {
    let fgb_reader = FgbReader::open(fgb).unwrap();
    let mut feature_iter = fgb_reader.select_all_seq().unwrap();
    let mut geos_writer = geozero::geos::GeosWriter::new();
    feature_iter.process(&mut geos_writer).unwrap();
    let geos0 = geos_writer.geometry();

    let fgb_reader = FgbReader::open(fgb).unwrap();
    let mut feature_iter = fgb_reader.select_all_seq().unwrap();
    let mut geos_writer = geozero::geos::GeosWriter::new();
    feature_iter.process(&mut geos_writer).unwrap();
    let geos1 = geos_writer.geometry();

    geos0.intersects(geos1).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let mut fgb_writer = FgbWriter::create("countries", GeometryType::Polygon).unwrap();

    let rdr = std::io::Cursor::new(data::WKT_DATA.as_bytes());
    let mut fin = BufReader::new(rdr);
    let mut reader = WktReader(&mut fin);

    reader.process(&mut fgb_writer).unwrap();

    let mut fgb: Vec<u8> = Vec::new();
    let mut fout = BufWriter::new(&mut fgb);
    fgb_writer.write(&mut fout).unwrap();
    drop(fout);
    c.bench_function("parse fgb", |b| b.iter(|| parse_fgb(fgb.as_slice())));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
