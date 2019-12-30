use average::{
    Estimate, Histogram, Max, MeanWithError, Merge, Min, Quantile, WeightedMean,
    WeightedMeanWithError,
};

fn main() {
    let mut max = average::Max::new();
    max.add(1.0);
    max.add(2.0);
    max.add(5.0);
    max.add(7.0);
    max.add(1.0);
    println!("Estimate: {}", max.estimate());

    let v: MeanWithError = (1..100)
        .map(f64::from)
        .collect::<MeanWithError>();
    println!("Mean = {} +- {}", v.mean(), v.error());
}
