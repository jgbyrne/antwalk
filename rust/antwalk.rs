use std::fmt;
use std::fs::File;
use std::io;
use std::io::Write;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Walk {
    n: usize,
    world: Vec<char>,
    ant: (usize, usize),
    fallen: bool,
}

impl Walk {
    fn new(n: usize) -> Walk {
        let mut walk = Walk {
            n: n,
            world: Vec::with_capacity(n * n),
            ant: (0, 0),
            fallen: false,
        };
        for _ in 0..(n * n) {
            walk.world.push('.');
        }
        walk.world[0] = '*';
        walk
    }

    fn walk(&mut self, direction: Direction) -> bool {
        let did_fall = match direction {
            Direction::Up => {
                if self.ant.1 == 0 {
                    true
                } else {
                    self.ant.1 -= 1;
                    false
                }
            }
            Direction::Down => {
                if self.ant.1 == (self.n - 1) {
                    true
                } else {
                    self.ant.1 += 1;
                    false
                }
            }
            Direction::Left => {
                if self.ant.0 == 0 {
                    true
                } else {
                    self.ant.0 -= 1;
                    false
                }
            }
            Direction::Right => {
                if self.ant.0 == (self.n - 1) {
                    true
                } else {
                    self.ant.0 += 1;
                    false
                }
            }
        };

        if did_fall {
            self.ant = (self.n + 1, self.n + 1);
            self.fallen = true;
        } else {
            self.world[self.ant.0 + (self.ant.1 * self.n)] = '*';
        }

        !did_fall
    }
}

impl fmt::Display for Walk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buf = String::new();
        for y in 0..self.n {
            for x in 0..self.n {
                if !self.fallen {
                    if (x, y) == self.ant {
                        buf.push('x');
                        continue;
                    }
                }
                buf.push(self.world[x + (y * self.n)]);
            }
            buf.push('\n');
        }
        write!(f, "{}", buf)
    }
}

fn antwalk() {
    println!("Welcome to AntWalk!");
    let mut walk = Walk::new(10);
    print!("{}", walk);

    loop {
        println!("[U]p, [D]own, [L]eft, or [R]ight");
        print!(">>> ");
        if io::stdout().flush().is_err() {
            println!("Could not flush stdin");
            return;
        }
        let mut command = String::new();
        match io::stdin().read_line(&mut command) {
            Ok(_) => (),
            Err(_) => {
                println!("Could not read stdin");
                return;
            }
        };

        let direction = match command.to_uppercase().trim() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => {
                println!("Command not recognised");
                continue;
            }
        };

        let result = walk.walk(direction);
        print!("{}", walk);

        if !result {
            println!("Walk has concluded");
            println!("Enter a filename to save your walk:");
            print!(">>> ");
            if io::stdout().flush().is_err() {
                println!("Could not flush stdin");
                return;
            }
            let mut filename = String::new();
            match io::stdin().read_line(&mut filename) {
                Ok(_) => (),
                Err(_) => {
                    println!("Could not read stdin");
                    return;
                }
            };

            let filename = filename.trim();

            let mut outf = match File::create(&filename) {
                Ok(f) => f,
                Err(_) => {
                    println!("Could not create file: {}", filename);
                    return;
                }
            };

            match outf.write_all(format!("{}", walk).as_bytes()) {
                Err(_) => {
                    println!("Could not write walk to {}", filename);
                }
                Ok(_) => {
                    println!("Walk successfully written to {}", filename);
                }
            }
            return;
        }
    }
}

fn main() {
    antwalk();
}
