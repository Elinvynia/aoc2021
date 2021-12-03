fn main() {
    let mut input = std::fs::read_to_string("input.txt").unwrap();
    input.pop(); // Remove newline at EoF

    let parts: Vec<String> = input.lines().map(|x| x.to_string()).collect();
    let length = parts[0].len();

    let mut gamma_string = String::new();
    for idx in 0..length {
        let mut zeroes = 0;
        let mut ones = 0;

        for part in &parts {
            let ch = part.chars().nth(idx).unwrap();
            if ch == '0' {
                zeroes += 1
            } else {
                ones += 1
            }
        }

        if zeroes > ones {
            gamma_string.push('0')
        } else {
            gamma_string.push('1')
        }
    }

    let mut epsilon_string = String::new();
    for x in gamma_string.chars() {
        if x == '0' {
            epsilon_string.push('1')
        } else {
            epsilon_string.push('0')
        }
    }

    let gamma = usize::from_str_radix(&gamma_string, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon_string, 2).unwrap();

    let mut candidates = parts.clone();
    while candidates.len() > 1 {
        for idx in 0..length {
            let mut to_remove = vec![];

            let mut zeroes = 0;
            let mut ones = 0;
            for x in &candidates {
                let ch = x.chars().nth(idx).unwrap();
                if ch == '0' {
                    zeroes += 1
                } else {
                    ones += 1
                }
            }

            let most_common = {
                if zeroes > ones {
                    '0'
                } else {
                    '1'
                }
            };

            for x in &candidates {
                let ch = x.chars().nth(idx).unwrap();
                if ch != most_common {
                    to_remove.push(x.clone())
                }
            }

            if to_remove.len() == candidates.len() {
                break;
            }

            candidates = candidates.into_iter().filter(|x| !to_remove.contains(x)).collect();
        }
    }

    let mut candidates_two = parts.clone();
    while candidates_two.len() > 1 {
        for idx in 0..length {
            let mut to_remove = vec![];

            let mut zeroes = 0;
            let mut ones = 0;
            for x in &candidates_two {
                let ch = x.chars().nth(idx).unwrap();
                if ch == '0' {
                    zeroes += 1
                } else {
                    ones += 1
                }
            }

            let most_common = {
                if zeroes > ones {
                    '0'
                } else {
                    '1'
                }
            };

            for x in &candidates_two {
                let ch = x.chars().nth(idx).unwrap();
                if ch == most_common {
                    to_remove.push(x.clone())
                }
            }

            if to_remove.len() == candidates_two.len() {
                break;
            }

            candidates_two = candidates_two.into_iter().filter(|x| !to_remove.contains(x)).collect();
        }
    }

    let oxygen = usize::from_str_radix(&candidates[0], 2).unwrap();
    let scrubber = usize::from_str_radix(&candidates_two[0], 2).unwrap();

    println!("Part one: {}", gamma * epsilon);
    println!("Part two: {}", oxygen * scrubber);
}
