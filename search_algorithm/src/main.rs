use const_random::const_random;

fn max_number(numbers: &[u64]) -> Option<u64> {
    let mut large = numbers.first()?;

    for number in numbers {
        if large < number {
            large = number
        }
    }

    Some(*large)
}

fn main() {
    static NUMBERS: [u64; 1_000_000] = [const_random!(u64); 1_000_000];
        
    benchmarking::warm_up();

    let benchmark_result = benchmarking::measure_function(|measurer| {
        measurer.measure(|| {
            max_number(&NUMBERS);
        });
    }).unwrap();

    println!("{:?}", benchmark_result.elapsed();
}
