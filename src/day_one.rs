use std::fs;

pub fn day_one() {
    let file_contents = fs::read_to_string("files/day_one/input.txt").unwrap();
    let elves: Vec<Vec<u32>> = file_contents
        .split("\n\n")
        .map(|elf| elf
            .lines()
            .map(|entry| entry.parse::<u32>().unwrap())
            .collect()
        ).collect();

    let mut elf_bags_sum: Vec<u32> = elves.iter().map(|elf| elf.iter().sum()).collect();
    elf_bags_sum.sort_by(|a, b| a.partial_cmp(b).unwrap());
    println!("largest bag: {}", elf_bags_sum.last().unwrap());

    let mut elf_bags_sum_clone = elf_bags_sum.clone();
    elf_bags_sum_clone.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let top_3_sum: u32 = elf_bags_sum_clone.iter().take(3).sum();
    println!("top 3 bags: {}", top_3_sum);
}