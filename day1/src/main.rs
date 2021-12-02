fn main() {
    let mut input = std::fs::read_to_string("input.txt").unwrap();
    input.pop(); // Newline at EOF
    let numbers: Vec<u64> = input.split("\n").map(|x| x.parse().unwrap()).collect();

    let mut counter = 0;
    for x in numbers.windows(2) {
        if x[0] < x[1] {
            counter += 1
        }
    }

    let sums: Vec<u64> = numbers.windows(3).map(|x| x.iter().sum()).collect();
    let mut counter_two = 0;
    for x in sums.windows(2) {
        if x[0] < x[1] {
            counter_two += 1
        }
    }

    println!("Part one: {}", counter);
    println!("Part two: {}", counter_two)
}
