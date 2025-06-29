fn main() {
    let txt = std::fs::read_to_string("../input.txt").unwrap();

    let mut res: i32 = 0;

    for (index, char) in txt.chars().enumerate() {
        match char {
            '(' => res += 1,
            ')' => res -= 1,
            _ => unreachable!(),
        }

        if res == -1 {
            let index = index + 1;
            print!("{index}");
            break;
        }
    }
}
