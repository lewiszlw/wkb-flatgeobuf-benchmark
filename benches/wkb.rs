use criterion::{criterion_group, criterion_main, Criterion};
use geos::Geom;
use geozero::wkb::{FromWkb, WkbDialect};
use geozero::{CoordDimensions, ToWkb};

mod data;

async fn parse_wkb(wkb: &[u8]) {
    let mut rdr = std::io::Cursor::new(&wkb[..]);
    let geos0 = geos::Geometry::from_wkb(&mut rdr, WkbDialect::Wkb).unwrap();

    let mut rdr = std::io::Cursor::new(&wkb[..]);
    let geos1 = geos::Geometry::from_wkb(&mut rdr, WkbDialect::Wkb).unwrap();

    geos0.intersects(&geos1).unwrap();
}

fn criterion_benchmark(c: &mut Criterion) {
    let wkt = geozero::wkt::Wkt(data::WKT_DATA);
    let wkb = wkt.to_wkb(CoordDimensions::xy()).unwrap();
    c.bench_function("parse wkb", |b| b.iter(|| parse_wkb(&wkb)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
