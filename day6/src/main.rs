use std::collections::HashMap;

fn main() {
    let mut input = std::fs::read_to_string("input.txt").unwrap();
    input.pop(); // Newline at EoF

    let state: Vec<i8> = input.split(",").map(|x| x.parse().unwrap()).collect();

    let mut state_map = HashMap::new();
    state_map.insert(0, 0);
    state_map.insert(1, 0);
    state_map.insert(2, 0);
    state_map.insert(3, 0);
    state_map.insert(4, 0);
    state_map.insert(5, 0);
    state_map.insert(6, 0);
    state_map.insert(7, 0);
    state_map.insert(8, 0);

    for x in state {
        *state_map.get_mut(&x).unwrap() += 1;
    };

    let mut part_one = state_map.clone();
    for _ in 0..80 {
        let eight = *part_one.get_mut(&8).unwrap();
        let seven = *part_one.get_mut(&7).unwrap();
        let six = *part_one.get_mut(&6).unwrap();
        let five = *part_one.get_mut(&5).unwrap();
        let four = *part_one.get_mut(&4).unwrap();
        let three = *part_one.get_mut(&3).unwrap();
        let two = *part_one.get_mut(&2).unwrap();
        let one = *part_one.get_mut(&1).unwrap();
        let zero = *part_one.get_mut(&0).unwrap();

        *part_one.get_mut(&8).unwrap() = zero;
        *part_one.get_mut(&7).unwrap() = eight;
        *part_one.get_mut(&6).unwrap() = seven + zero;
        *part_one.get_mut(&5).unwrap() = six;
        *part_one.get_mut(&4).unwrap() = five;
        *part_one.get_mut(&3).unwrap() = four;
        *part_one.get_mut(&2).unwrap() = three;
        *part_one.get_mut(&1).unwrap() = two;
        *part_one.get_mut(&0).unwrap() = one;
    }

    let mut part_two = state_map.clone();
    for _ in 0..256 {
        let eight = *part_two.get_mut(&8).unwrap();
        let seven = *part_two.get_mut(&7).unwrap();
        let six = *part_two.get_mut(&6).unwrap();
        let five = *part_two.get_mut(&5).unwrap();
        let four = *part_two.get_mut(&4).unwrap();
        let three = *part_two.get_mut(&3).unwrap();
        let two = *part_two.get_mut(&2).unwrap();
        let one = *part_two.get_mut(&1).unwrap();
        let zero = *part_two.get_mut(&0).unwrap();

        *part_two.get_mut(&8).unwrap() = zero;
        *part_two.get_mut(&7).unwrap() = eight;
        *part_two.get_mut(&6).unwrap() = seven + zero;
        *part_two.get_mut(&5).unwrap() = six;
        *part_two.get_mut(&4).unwrap() = five;
        *part_two.get_mut(&3).unwrap() = four;
        *part_two.get_mut(&2).unwrap() = three;
        *part_two.get_mut(&1).unwrap() = two;
        *part_two.get_mut(&0).unwrap() = one;
    }

    println!("Part one: {}", part_one.values().sum::<u64>());
    println!("Part two: {}", part_two.values().sum::<u64>());
}
