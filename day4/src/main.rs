#[derive(Debug, Clone, Eq, PartialEq)]
struct Bingo {
    inner: Vec<Vec<(u64, bool)>>,
}

impl Bingo {
    fn mark(&mut self, to_mark: u64) {
        for row in &mut self.inner {
            for (number, marked) in row {
                if *number == to_mark {
                    *marked = true
                }
            }
        }
    }

    fn is_completed(&self) -> bool {
        // Check rows
        for row in &self.inner {
            let mut marks = 0;

            for (_, marked) in row {
                if *marked {
                    marks += 1
                }
            }

            if marks == 5 {
                return true
            }
        }

        // Check columns
        for column in 0..5 {
            let mut marks = 0;

            for row in &self.inner {
                let (_, marked) = row[column];
                if marked {
                    marks += 1
                }
            }

            if marks == 5 {
                return true;
            }
        }


        return false
    }

    fn get_unmarked(&self) -> Vec<u64> {
        let mut unmarked = vec![];

        for row in &self.inner {
            for (num, marked) in row {
                if !*marked {
                    unmarked.push(*num)
                }
            }
        }

        return unmarked;
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let draws: Vec<u64> = input.lines().nth(0).map(|x| x.split(",").map(|x| x.parse().unwrap())).unwrap().collect();

    let mut bingos: Vec<Bingo> = vec![];
    let mut current = vec![];
    for x in input.lines().skip(2) {
        if x.chars().all(|x| x.is_whitespace()) {
            let bingo = Bingo {
                inner: current.clone()
            };
            bingos.push(bingo);
            current.clear();
            continue;
        }

        let line: Vec<(u64, bool)> = x.split_whitespace().map(|x| (x.parse().unwrap(), false)).collect();
        current.push(line);
    }

    let mut bingos_clone = bingos.clone();
    let mut first_current_draw;
    let winning_bingo;

    'main: loop  {
        for draw in &draws {
            first_current_draw = draw;

            for bingo in bingos_clone.iter_mut() {
                bingo.mark(*draw);

                if bingo.is_completed() {
                    winning_bingo = bingo.clone();
                    break 'main;
                }
            }
        }
    }

    let first_unmarked_sum: u64 = winning_bingo.get_unmarked().into_iter().sum();


    let mut bingos_clone = bingos.clone();
    let len = bingos_clone.len();
    let mut second_current_draw = 0;
    let mut completed = vec![];

    for draw in &draws {
        second_current_draw = *draw;

        for bingo in bingos_clone.iter_mut() {
            bingo.mark(*draw);

            if bingo.is_completed() {
                completed.push(bingo.clone())
            }
        }

        if completed.len() == len {
            break;
        }

        bingos_clone = bingos_clone.into_iter().filter(|b| !b.is_completed()).collect();
    }

    let second_unmarked_sum: u64 = completed.last().unwrap().get_unmarked().into_iter().sum();

    println!("Part one: {}", first_unmarked_sum * first_current_draw);
    println!("Part two: {}", second_unmarked_sum * second_current_draw);
}
