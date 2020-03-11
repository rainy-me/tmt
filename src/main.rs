use std::env;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    let nums = [
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
    ];
    let colon = "
   ██



   ██
";
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
        match secs {
            0 => {
                secs = 59;
                match time {
                    0 => break,
                    _ => time -= 1,
                }
            }
            _ => {
                sleep(one_sec);
                secs -= 1;
            }
        }
        let mut clock = vec![String::new(); 7];
        for c in [
            nums[time / 10],
            nums[time % 10],
            colon,
            nums[secs / 10],
            nums[secs % 10],
        ]
        .iter()
        {
            let lines: Vec<&str> = c.split("\n").collect();
            for (i, line) in clock.iter_mut().enumerate() {
                line.push_str(&format!(" {:<8}", lines[i]));
            }
        }
        print!("\x1B[2J");
        println!("{:}", clock.join("\n"));
    }
}
