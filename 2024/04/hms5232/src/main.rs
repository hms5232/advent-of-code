fn main() {
    // read content from file input.txt
    let file_path = "input.txt"; // change this to the path of your file
    let contents = std::fs::read_to_string(file_path).expect("File Read error");

    // split content line by line into vector
    // and split each line to vector that every alphabet is one element
    let lines = contents
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    // https://adventofcode.com/2024/day/4
    // search each word as a starting point
    let part1_answer = lines
        .iter()
        .enumerate()
        .map(|(index_x, each_x)| {
            each_x
                .iter()
                .enumerate()
                .map(|(index_y, each_y)| {
                    const WORD: [char; 4] = ['X', 'M', 'A', 'S'];
                    let mut target = Vec::new();
                    // only check if the word is the first or the last
                    if check_by_coordinate(&lines, WORD[0], index_x, index_y) {
                        target = WORD.to_vec();
                    } else if check_by_coordinate(&lines, WORD[3], index_x, index_y) {
                        target = WORD.to_vec().iter().rev().map(|x| *x).collect();
                    } else {
                        return 0;
                    }

                    let mut this_round = get_around_coordinate(&lines, index_x, index_y);
                    let mut next_round = Vec::new();
                    // the first and last words don't need to be checked
                    for i in 1..(target.len() - 1) {
                        this_round.iter().for_each(|(rx, ry)| {
                            if check_by_coordinate(&lines, target[i], *rx, *ry) {
                                get_next_check(&lines, &target[i + 1], *rx, *ry)
                                    .iter()
                                    .for_each(|(x, y)| next_round.push((*x, *y)));
                            }
                        });
                        this_round = next_round.clone();
                        next_round = Vec::new();
                    }
                    this_round.len()
                })
                .sum::<usize>()
        })
        .sum::<usize>()
        / 2;

    println!("Part 1: {}", part1_answer);
}

/// Check whether the word is in the matrix on specified position
fn check_by_coordinate(lines: &Vec<Vec<char>>, word: char, x: usize, y: usize) -> bool {
    if let Some(y_axis) = lines.get(y) {
        if let Some(x_axis) = y_axis.get(x) {
            if x_axis == &word {
                return true;
            }
        }
    }
    false
}

/// Get all coordinates around the specified coordinate
fn get_around_coordinate(lines: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<(usize, usize)> {
    // get all coordinates around the specified coordinate
    let mut coordinates = Vec::new();
    if y != 0 {
        coordinates.push((x, y - 1)); // top
        coordinates.push((x + 1, y - 1)); // top right
    }
    if x != 0 {
        coordinates.push((x - 1, y)); // left
        coordinates.push((x - 1, y + 1)); // bottom left
    }
    if x != 0 && y != 0 {
        coordinates.push((x - 1, y - 1)); // top left
    }
    coordinates.push((x + 1, y)); // right
    coordinates.push((x, y + 1)); // bottom
    coordinates.push((x + 1, y + 1)); // bottom right

    // check whether the coordinate is in the matrix => valid
    let mut result = Vec::new();
    coordinates.iter().for_each(|(candidate_x, candidate_y)| {
        if let Some(y_axis) = lines.get(*candidate_y) {
            if let Some(_x_axis) = y_axis.get(*candidate_x) {
                result.push((*candidate_x, *candidate_y));
            }
        }
    });
    result
}

fn get_next_check(lines: &Vec<Vec<char>>, word: &char, x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut next_check = Vec::new();
    get_around_coordinate(&lines, x, y)
        .iter()
        .for_each(|(x_coordinate, y_coordinate)| {
            if check_by_coordinate(lines, *word, *x_coordinate, *y_coordinate) {
                next_check.push((*x_coordinate, *y_coordinate));
            }
        });
    next_check
}
