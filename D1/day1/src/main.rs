use regex::Regex;

fn main() {
    process();
    process_p2();
}

fn process() -> String {
    let mut input = include_str!("../input1.txt");
    let re = Regex::new(r"[^0-9\n]");
    let binding = re.unwrap().replace_all(input, ""); // Remove all text but numbers.
    dbg!(&binding);

    let mut sum = 0;
    for l in binding.lines() {
        if l.len() < 2 {
            let first = l.chars().nth(0).unwrap().to_digit(10).unwrap();
            let last = l.chars().nth(l.len() - 1).unwrap().to_digit(10).unwrap();
            sum += (String::from(first.to_string()) + String::from(last.to_string()).as_str())
                .parse::<u32>()
                .unwrap();

            println!(
                "{}",
                String::from(
                    (String::from(first.to_string()) + String::from(last.to_string()).as_str())
                        .parse::<u32>()
                        .unwrap()
                        .to_string()
                )
            )
        } else {
            let first = l.chars().nth(0).unwrap().to_digit(10).unwrap();
            let last = l.chars().nth(l.len() - 1).unwrap().to_digit(10).unwrap();
            sum += (String::from(first.to_string()) + String::from(last.to_string()).as_str())
                .parse::<u32>()
                .unwrap();

            println!(
                "{}",
                String::from(
                    (String::from(first.to_string()) + String::from(last.to_string()).as_str())
                        .parse::<u32>()
                        .unwrap()
                        .to_string()
                )
            )
        }
    }

    dbg!(sum);

    binding.to_string()
}

fn process_p2() -> String {
    let mut input = include_str!("../input1.txt");
    //    let mut input = String::from(
    //        "two1nine
    //eightwothree
    //abcone2threexyz
    //xtwone3four
    //4nineeightseven2
    //zoneight234
    //7pqrstsixteen",
    //    );

    let sum = input.lines().map(process_line).sum::<u32>();

    dbg!(sum);

    sum.to_string()
}

fn process_line(line: &str) -> u32 {
    let mut it = (0..line.len()).filter_map(|index| {
        let sub_line = &line[index..];
        let result = if sub_line.starts_with("one") {
            '1'
        } else if sub_line.starts_with("two") {
            '2'
        } else if sub_line.starts_with("three") {
            '3'
        } else if sub_line.starts_with("four") {
            '4'
        } else if sub_line.starts_with("five") {
            '5'
        } else if sub_line.starts_with("six") {
            '6'
        } else if sub_line.starts_with("seven") {
            '7'
        } else if sub_line.starts_with("eight") {
            '8'
        } else if sub_line.starts_with("nine") {
            '9'
        } else {
            sub_line.chars().next().unwrap()
        };

        result.to_digit(10)
    });
    let first = it.next().expect("First must be a number.");

    match it.last() {
        Some(num) => format!("{first}{num}"),
        None => format!("{first}{first}"),
    }
    .parse::<u32>()
    .expect("Last must now be a number. I hope...")
}
