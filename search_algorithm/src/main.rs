fn max_number(numbers: &[i64]) -> Option<i64> {
    let mut large = numbers.first()?;

    for number in numbers {
        if large < number {
            large = number
        }
    }

    Some(*large)
}

fn main() {
    let numbers: [i64; 10] = [5, 3, 7, 8, 6, 1, 2, 9, 10, 4];
    let biggest = max_number(&numbers).unwrap();

    println!("Biggest number: {}", biggest);
}
