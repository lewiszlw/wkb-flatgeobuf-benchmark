use criterion::{criterion_group, criterion_main, Criterion};
use geos::Geom;
use geozero::wkb::{FromWkb, WkbDialect};
use geozero::{CoordDimensions, ToWkb};

mod data;

async fn parse_ewkb(ewkb: &[u8]) {
    let mut rdr = std::io::Cursor::new(&ewkb[..]);
    let geos0 = geos::Geometry::from_wkb(&mut rdr, WkbDialect::Ewkb).unwrap();

    let mut rdr = std::io::Cursor::new(&ewkb[..]);
    let geos1 = geos::Geometry::from_wkb(&mut rdr, WkbDialect::Ewkb).unwrap();

    geos0.intersects(&geos1).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let wkt = geozero::wkt::Wkt(data::WKT_DATA);
    let ewkb = wkt.to_ewkb(CoordDimensions::xy(), None).unwrap();
    c.bench_function("parse ewkb", |b| b.iter(|| parse_ewkb(&ewkb)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
