use std::io;

fn read_integer() -> Vec<i32>
{
    let mut numbers = String::new();
    io::stdin().read_line(&mut numbers).ok().expect("read error");
    let numbers: Vec<i32> =
        numbers.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
    return numbers;
}


fn main() {
    let magic_numbers = vec![2, 3, 9];
    let test_cases = read_integer();
    for _index in 0..test_cases[0] {
        let start_end_numbers = read_integer();
        let start_number = start_end_numbers[0];
        let end_number = start_end_numbers[1];
        let mut count = 0;
        for val in start_number..end_number + 1 {
            let last_digit = val % 10;
            if magic_numbers.contains(&last_digit) {
                count += 1;
            };
        }
        println!("{}", count);
    }
}
