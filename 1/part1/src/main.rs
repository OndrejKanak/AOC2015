fn main() {
    let txt = std::fs::read_to_string("../input.txt").unwrap();

    let mut res = 0;

    for char in txt.chars() {
        match char {
            '(' => res += 1,
            ')' => res -= 1,
            _ => unreachable!(),
        }
    }
    print!("{res}");
}
