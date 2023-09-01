pub mod days;
pub mod utils;

fn main() {
    let file_content = utils::load_file("data/day_1.txt");
    let result = days::day_1::part_1(&file_content);
    print!("{}", result);
}
