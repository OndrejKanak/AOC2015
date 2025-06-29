fn main() {
    let txt = std::fs::read_to_string("../input.txt").unwrap();
    let lines = txt.lines();
    let mut res = 0;

    for line in lines {
        if line.is_empty() {
            break;
        }
        let mut line = line.split('x');
        let length = line.next().unwrap().parse::<u32>().unwrap();
        let width = line.next().unwrap().parse::<u32>().unwrap();
        let height = line.next().unwrap().parse::<u32>().unwrap();

        let side1 = length * width;
        let side2 = length * height;
        let side3 = width * height;

        res += 2 * side1 + 2 * side2 + 2 * side3;

        if side1 <= side2 && side1 <= side3 {
            res += side1;
        } else if side2 <= side1 && side2 <= side3 {
            res += side2;
        } else {
            res += side3;
        }
    }
    print!("{res}");
}
