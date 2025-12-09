fn main() {
    // read content from file input.txt
    let file_path = "input.txt"; // change this to the path of your file
    let contents = std::fs::read_to_string(file_path).expect("File Read error");

    let mut part1 = 0;
    let mut part2 = 0;

    // split content into lines
    let banks = contents.lines();
    for bank in banks {
        let joltages = bank
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect::<Vec<u32>>();
        // 找出最大
        let max = joltages.iter().max().unwrap();
        let max_index = joltages.iter().position(|&x| x == *max).unwrap();
        // 以最大的數字為基準分割成向量，兩邊分別找出最大的
        let mut joltages_remove_max = joltages.clone();
        joltages_remove_max.remove(max_index);
        let (left, right) = joltages_remove_max.split_at(max_index);
        let left_max = left.iter().max().unwrap_or(&0);
        let left_max = format!("{left_max}{max}").parse::<u32>().unwrap();
        let right_max = right.iter().max();
        let right_max = if right_max.is_some() {
            format!("{max}{}", right_max.unwrap())
                .parse::<u32>()
                .unwrap()
        } else {
            format!("{max}").parse::<u32>().unwrap()
        };
        part1 += vec![left_max, right_max].iter().max().unwrap();

        /* part 2 */
        // part 2 我決定換個方式，用 argsort() 的輔助
        let mut argsorted = argsort(&joltages);
        dbg!(&joltages, &argsorted);
        argsorted.reverse();
        _ = argsorted.split_off(12);
        argsorted.sort();
        // 串接起來
        let mut result = String::new();
        argsorted.iter().for_each(|index| result = format!("{result}{}", joltages.get(*index).unwrap()));
        part2 += result.parse::<usize>().unwrap();
    }
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2); // FIXME: 234234234234278 選擇後是 343434234278，但有更好的選擇 434234234278
}

/// Source - https://stackoverflow.com/a
/// Posted by kmdreko
/// Retrieved 2025-12-05, License - CC BY-SA 4.0
pub fn argsort<T: Ord>(data: &[T]) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| &data[i]);
    indices
}
