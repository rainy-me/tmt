use std::env;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    let numbers: Vec<Vec<String>> = [
        "
███████
██   ██
██   ██
██   ██
███████
",
        "
   ██
 ████
   ██
   ██
████████
",
        "
███████
     ██
███████
██
███████
",
        "
███████
     ██
███████
     ██
███████
",
        "
██   ██
██   ██
███████
     ██
     ██
",
        "
███████
██
███████
     ██
███████
",
        "
███████
██
███████
██   ██
███████
",
        "
███████
     ██
    ██
  ██
 ██
",
        "
███████
██   ██
███████
██   ██
███████
",
        "
███████
██   ██
███████
     ██
███████
",
    ]
    .iter()
    .map(|character| transform_to_lines(character))
    .collect();
    let colon = transform_to_lines(
        "
   ██



   ██
 ",
    );
    let mut mins: usize = match args.len() {
        2 => match args[1].parse() {
            Ok(t) => t,
            _ => panic!(),
        },
        _ => 15,
    };
    let mut secs = 0;
    let one_sec = Duration::from_secs(1);

    loop {
        let mut clock = vec![String::new(); 7];
        for source_lines in [
            &numbers[mins / 10],
            &numbers[mins % 10],
            &colon,
            &numbers[secs / 10],
            &numbers[secs % 10],
        ]
        .iter()
        {
            for (i, clock_line) in clock.iter_mut().enumerate() {
                clock_line.push_str(&source_lines[i]);
            }
        }
        print!("\x1B[2J{}", clock.join("\n"));
        sleep(one_sec);
        match secs {
            0 => {
                secs = 59;
                match mins {
                    0 => break,
                    _ => mins -= 1,
                }
            }
            _ => {
                secs -= 1;
            }
        }
    }
}

fn transform_to_lines(character: &str) -> Vec<String> {
    character
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| format!(" {:<8}", line))
        .collect()
}
