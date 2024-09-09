#![allow(unused_must_use)]
#![allow(clippy::result_unit_err)]

/// Don't mind this, just playing around
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use transmute_cve::transmute;
#[allow(clippy::transmute_int_to_float)]
#[repr(u8)]
pub enum ReprTest {
    One = 0,
    Two,
    Three,
}

// When not including OOB integers, all this compiles to the same asm
// Interesting..
pub fn convert_std(value: u8) -> Result<ReprTest, ()> {
    if value > 0 && value < 4 {
        let value: ReprTest = unsafe { std::mem::transmute(value) };
        return Ok(value);
    }
    Err(())
}

pub fn convert_safe(value: u8) -> Result<ReprTest, ()> {
    if value > 0 && value < 4 {
        let value: ReprTest = transmute(value);
        return Ok(value);
    }
    Err(())
}

impl std::convert::TryFrom<u8> for ReprTest {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            _ => Err(()),
        }
    }
}

fn bench_tryfrom(c: &mut Criterion) {
    c.bench_function("TryFrom conversion", |b| {
        b.iter(|| {
            for i in 0..5 {
                black_box(ReprTest::try_from(black_box(i)));
            }
        })
    });
}

fn bench_std_transmute(c: &mut Criterion) {
    c.bench_function("std::mem::transmute conversion", |b| {
        b.iter(|| {
            for i in 0..5 {
                black_box(convert_std(black_box(i)));
            }
        })
    });
}

fn bench_safe_transmute(c: &mut Criterion) {
    c.bench_function("safe transmute conversion", |b| {
        b.iter(|| {
            for i in 0..5 {
                black_box(convert_safe(black_box(i)));
            }
        })
    });
}

criterion_group!(
    benches,
    bench_std_transmute,
    bench_safe_transmute,
    bench_tryfrom
);
criterion_main!(benches);
