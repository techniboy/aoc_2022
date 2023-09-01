fn sum_of_elf_cal(elf_cals: &str) -> u32 {
    return elf_cals.lines().map(|x| x.parse::<u32>().unwrap()).sum();
}

pub fn part_1(input: &str) -> u32 {
    let all_elf_cals: Vec<&str> = input.split("\n\n").collect();
    let cals_per_elf: Vec<u32> = all_elf_cals.iter().map(|x| sum_of_elf_cal(*x)).collect();
    return *cals_per_elf.iter().max().unwrap();
}
