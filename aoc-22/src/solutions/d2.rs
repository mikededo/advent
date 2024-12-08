use utils::read_lines;

pub fn solve_a() {
    let res = read_lines("d2.txt", 22).iter().fold(0, |acc, line| {
        let game = line.split(" ").collect::<Vec<&str>>();
        acc + match game[0] {
            "A" => match game[1] {
                "X" => 4,
                "Y" => 8,
                "Z" => 3,
                _ => panic!("Invalid option."),
            },
            "B" => match game[1] {
                "X" => 1,
                "Y" => 5,
                "Z" => 9,
                _ => panic!("Invalid option."),
            },
            "C" => match game[1] {
                "X" => 7,
                "Y" => 2,
                "Z" => 6,
                _ => panic!("Invalid option."),
            },
            _ => panic!("Invalid option."),
        }
    });

    println!("{res}");
}

pub fn solve_b() {
    let res = read_lines("d2.txt", 22).iter().fold(0, |acc, line| {
        let game = line.split(" ").collect::<Vec<&str>>();
        acc + match game[0] {
            "A" => match game[1] {
                "X" => 3,
                "Y" => 4,
                "Z" => 8,
                _ => panic!("Invalid option."),
            },
            "B" => match game[1] {
                "X" => 1,
                "Y" => 5,
                "Z" => 9,
                _ => panic!("Invalid option."),
            },
            "C" => match game[1] {
                "X" => 2,
                "Y" => 6,
                "Z" => 7,
                _ => panic!("Invalid option."),
            },
            _ => panic!("Invalid option."),
        }
    });

    println!("{res}");
}
