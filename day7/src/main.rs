fn main() {
    let mut input = std::fs::read_to_string("input.txt").unwrap();
    input.pop(); // Newline at EoF

    let positions: Vec<isize> = input.split(",").map(|x| x.parse().unwrap()).collect();

    let mut cheapest_cost = positions.iter().sum();
    for target in &positions {
        let mut cost = 0;
        for pos in &positions {
            cost += (target - pos).abs();
        }
        if cost < cheapest_cost {
            cheapest_cost = cost;
        }
    }

    let mut cheapest_cost_two = positions.iter().map(|x| x * 10000).sum();
    let max = *positions.iter().max().unwrap();
    for target in 0..max {
        let mut cost = 0;
        for pos in &positions {
            let num = (target - pos).abs() + 1;
            for x in 1..num {
                cost += x;
            }
        }
        if cost < cheapest_cost_two {
            cheapest_cost_two = cost;
        }
    }

    println!("Part one: {}", cheapest_cost);
    println!("Part two: {}", cheapest_cost_two);
}
