fn generate_points(line: Vec<usize>) -> Vec<(usize, usize)> {
    let mut points = vec![];

    let (mut x1, mut y1, x2, y2) = (line[0] as isize, line[1] as isize, line[2] as isize, line[3] as isize);

    points.push((x1 as usize, y1 as usize));
    points.push((x2 as usize, y2 as usize));

    let step_x: isize = {
        if x1 > x2 {
            -1
        } else if x1 < x2 {
            1
        } else {
            0
        }
    };

    let step_y: isize = {
        if y1 > y2 {
            -1
        } else if y1 < y2 {
            1
        } else {
            0
        }
    };

    while (x1, y1) != (x2, y2) {
        x1 += step_x;
        y1 += step_y;

        points.push((x1 as usize, y1 as usize));
    }

    points.sort();
    points.dedup();

    return points
}

fn main() {
    let mut input = std::fs::read_to_string("input.txt").unwrap();
    input.pop(); // Newline at EoF

    let mut lines = vec![];

    for line in input.lines() {
        let coords: Vec<usize> = line.replace(" -> ", ",").split(",").map(|x| x.parse().unwrap()).collect();
        lines.push(coords)
    }

    let mut high_start = 0;
    for line in &lines {
        if line[0] > high_start {
            high_start = line[0]
        }

        if line[1] > high_start {
            high_start = line[1]
        }
    }
    high_start += 1; // zero index

    let mut high_end = 0;
    for line in &lines {
        if line[2] > high_end {
            high_end = line[2]
        }

        if line[3] > high_end {
            high_end = line[3]
        }
    }
    high_end += 1; // zero index

    let mut field = vec![vec![0u64; high_end]; high_start];
    let mut field_two = field.clone();

    let mut valid_lines = vec![];
    for line in &lines {
        if line[0] == line[2] {
            valid_lines.push(line.clone());
        } else if line[1] == line[3] {
            valid_lines.push(line.clone());
        }
    }

    for line in valid_lines {
        let points = generate_points(line);
        for point in points {
            field[point.0][point.1] += 1
        }
    }

    let mut two_or_more = 0;
    for row in &field {
        for column in row {
            if *column >= 2 {
                two_or_more += 1
            }
        }
    }

    for line in lines {
        let points = generate_points(line);
        for point in points {
            field_two[point.0][point.1] += 1
        }
    }

    let mut two_or_more_two = 0;
    for row in &field_two {
        for column in row {
            if *column >= 2 {
                two_or_more_two += 1
            }
        }
    }


    println!("Part one: {}", two_or_more);
    println!("Part two: {}", two_or_more_two);
}
