use criterion::{criterion_group, criterion_main, Criterion};
use uuid::Uuid;

fn uuid_v4_cmp_string(c: &mut Criterion) {
    c.bench_function("uuid_v4_cmp_string", |b| {
        let uuid_v4 = Uuid::new_v4();
        let s = uuid_v4.to_string();
        b.iter(|| optimizing_serde_slash_rust_match_statement::uuid_v4_cmp_string(&s));
    });
}

fn uuid_v4_cmp_bytes(c: &mut Criterion) {
    c.bench_function("uuid_v4_cmp_bytes", |b| {
        let uuid_v4 = Uuid::new_v4();
        let s = uuid_v4.to_string();
        b.iter(|| optimizing_serde_slash_rust_match_statement::uuid_v4_cmp_bytes(&s));
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default();
    targets =
        uuid_v4_cmp_string,
        uuid_v4_cmp_bytes,

);
criterion_main!(benches);
