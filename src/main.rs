use std::env;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    let nums: Vec<Vec<String>> = [
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
    let mut time: usize = match args.len() {
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
            &nums[time / 10],
            &nums[time % 10],
            &colon,
            &nums[secs / 10],
            &nums[secs % 10],
        ]
        .iter()
        {
            for (i, line) in clock.iter_mut().enumerate() {
                line.push_str(&source_lines[i]);
            }
        }
        print!("\x1B[2J{}", clock.join("\n"));
        sleep(one_sec);
        match secs {
            0 => {
                secs = 59;
                match time {
                    0 => break,
                    _ => time -= 1,
                }
            }
            _ => {
                secs -= 1;
            }
        }
    }
}

fn transform_to_lines(line: &str) -> Vec<String> {
    line.split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|line| format!(" {:<8}", line))
        .collect()
}
