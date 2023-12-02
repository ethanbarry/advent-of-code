use std::collections::HashMap;

fn main() {
    process();
    process2();
}

fn process() {
    let input = include_str!("../input.txt");

    //     let input = String::from(
    //         "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    // Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    // Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    // Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    // Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
    //     );

    let mut sum: u32 = 0;

    for l in input.lines() {
        // Get the game ID from before the ':'
        let id: u32 = l
            .split_once(":")
            .expect("WHERE IS MY COLON!?")
            .0
            .split_whitespace()
            .last()
            .expect("SHOULD BE A LAST")
            .parse()
            .expect("AAAAH!");
        dbg!(&l, &id);

        let game_string = l.split_once(":").expect("WHERE IS MY COLON!?").1;

        let mut hm: HashMap<&str, u8> = HashMap::new();

        for s in game_string.split(";") {
            dbg!(&s.trim());

            for t in s.split(",") {
                for u in t.split_whitespace() {
                    match u.trim() {
                        "red" => {
                            let val = t
                                .split_whitespace()
                                .nth(0)
                                .expect("SHOULD BE A NUM. HERE!")
                                .parse::<u8>()
                                .expect("SHOULD PARSE!");
                            if hm.get("RED").unwrap_or(&0) < &val {
                                hm.insert("RED", val);
                            }
                            println!("RED {}", val.to_string());
                        }
                        "green" => {
                            let val = t
                                .split_whitespace()
                                .nth(0)
                                .expect("SHOULD BE A NUM. HERE!")
                                .parse::<u8>()
                                .expect("SHOULD PARSE!");
                            if hm.get("GREEN").unwrap_or(&0) < &val {
                                hm.insert("GREEN", val);
                            }
                            println!("GREEN {}", val.to_string());
                        }
                        "blue" => {
                            let val = t
                                .split_whitespace()
                                .nth(0)
                                .expect("SHOULD BE A NUM. HERE!")
                                .parse::<u8>()
                                .expect("SHOULD PARSE!");
                            if hm.get("BLUE").unwrap_or(&0) < &val {
                                hm.insert("BLUE", val);
                            }
                            println!("BLUE {}", val.to_string());
                        }
                        _ => {}
                    }
                }
            }

            dbg!(&hm); // hm contains the highest values seen for each.
        }

        // Test conditions.
        if hm.get("RED").unwrap_or(&0) > &12
            || hm.get("GREEN").unwrap_or(&0) > &13
            || hm.get("BLUE").unwrap_or(&0) > &14
        {
        } else {
            println!("ADDING {} TO SUM!!", id);
            sum = sum + id;
        }
    }

    dbg!(&sum);
}
/* PART II */

fn process2() {
    let input = include_str!("../input.txt");

    let mut sum: u32 = 0;

    for l in input.lines() {
        // Get the game ID from before the ':'
        // NOTE Line ID no longer relevant in part II.
        let id: u32 = l
            .split_once(":")
            .expect("WHERE IS MY COLON!?")
            .0
            .split_whitespace()
            .last()
            .expect("SHOULD BE A LAST")
            .parse()
            .expect("AAAAH!");
        dbg!(&l, &id);

        // NOTE No longer relevant.
        let game_string = l.split_once(":").expect("STOP LOSING MY COLON!?").1;

        // Stores color and highest value in a line.
        let mut hm: HashMap<&str, u8> = HashMap::new();

        for s in game_string.split(";") {
            dbg!(&s.trim());

            for t in s.split(",") {
                for u in t.split_whitespace() {
                    match u.trim() {
                        "red" => {
                            let val = t
                                .split_whitespace()
                                .nth(0)
                                .expect("SHOULD BE A NUM. HERE!")
                                .parse::<u8>()
                                .expect("SHOULD PARSE!");
                            if hm.get("RED").unwrap_or(&0) < &val {
                                hm.insert("RED", val);
                            }
                            println!("RED {}", val.to_string());
                        }
                        "green" => {
                            let val = t
                                .split_whitespace()
                                .nth(0)
                                .expect("SHOULD BE A NUM. HERE!")
                                .parse::<u8>()
                                .expect("SHOULD PARSE!");
                            if hm.get("GREEN").unwrap_or(&0) < &val {
                                hm.insert("GREEN", val);
                            }
                            println!("GREEN {}", val.to_string());
                        }
                        "blue" => {
                            let val = t
                                .split_whitespace()
                                .nth(0)
                                .expect("SHOULD BE A NUM. HERE!")
                                .parse::<u8>()
                                .expect("SHOULD PARSE!");
                            if hm.get("BLUE").unwrap_or(&0) < &val {
                                hm.insert("BLUE", val);
                            }
                            println!("BLUE {}", val.to_string());
                        }
                        _ => {}
                    }
                }
            }

            dbg!(&hm); // hm contains the highest values seen for each.
        }

        // Add power to sum.
        let power = *hm.get("RED").unwrap_or(&0) as u64
            * *hm.get("GREEN").unwrap_or(&0) as u64
            * *hm.get("BLUE").unwrap_or(&0) as u64;

        sum += power as u32;
    }

    dbg!(&sum);
}
