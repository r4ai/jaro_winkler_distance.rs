#[macro_use]
extern crate criterion;

use std::collections::HashMap;

use criterion::black_box;
use criterion::Criterion;

use jaro_winkler_distance::jaro_winkler_distance;
use jaro_winkler_distance::PrefixLength;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("jaro_winkler_distance");
    let software_names: HashMap<u32, &str> = [
        (3, "Git"),
        (5, "nginx"),
        (8, "PostgreSQL"),
        (10, "TensorFlow"),
        (15, "Apache Cassandra"),
        (20, "Microsoft Power BI"),
        (30, "Amazon Elastic MapReduce (EMR)"),
        (40, "Google Cloud Machine Learning Engine for"),
        (50, "IBM Cloud Pak Data and Oracle Cloud Infrastructure"),
    ]
    .iter()
    .cloned()
    .collect();

    let search_words: HashMap<u32, &str> = [
        (3, "igt"),
        (5, "ngobx"),
        (8, "PosrgerSqO"),
        (10, "TebsprFlow"),
        (15, "Apacbe Caeeendwa"),
        (20, "Mivrosovt Power NI"),
        (30, "AmazobnErastic MopReduce (ENR)"),
        (40, "Gooele Cloud Macbnie Learning Ebgine foe"),
        (50, "IBM Crod Park Dete and Orakle Croud Infravtructara"),
    ]
    .iter()
    .cloned()
    .collect();

    for len in [3, 5, 8, 10, 15, 20, 30, 40, 50].iter() {
        let len_display = if len < &10 {
            format!("0{len}")
        } else {
            format!("{len}")
        };
        let lhs = search_words.get(len).unwrap();
        let rhs = software_names.get(len).unwrap();

        group.bench_function(format!("jaro_winkler_distance_{len_display}"), |b| {
            b.iter(|| jaro_winkler_distance(black_box(lhs), black_box(rhs), &PrefixLength::Four))
        });
    }
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(100);
    targets = criterion_benchmark
);
criterion_main!(benches);
