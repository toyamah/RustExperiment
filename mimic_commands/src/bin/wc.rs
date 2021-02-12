use std::fs::File;
use std::io::{Read, Write};
use std::process::exit;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        io::stderr()
            .write_all(format!("Specify only one file\n").as_ref())
            .ok();
        exit(1);
    }

    let path = &args[1];
    match File::open(path) {
        Ok(f) => {
            let count = count(f);
            io::stdout()
                .write_all(
                    format!(
                        "{:>8} {:>7} {:>7} {}\n",
                        count.lines, count.words, count.bytes, path
                    )
                    .as_ref(),
                )
                .ok();
        }
        Err(_) => {
            io::stderr()
                .write_all(format!("wc: {}: open: No such file or directory\n", path).as_ref())
                .ok();
            exit(1);
        }
    }
}

struct Count {
    bytes: usize,
    lines: usize,
    words: usize,
}

fn count(mut f: File) -> Count {
    let mut buffer = [0; 1024 * 4];
    let mut byte_count: usize = 0;
    let mut line_count: usize = 0;
    let mut word_count: usize = 0;
    let mut in_word = false;

    loop {
        let size = f.read(&mut buffer[..]).expect("failed to read");
        if size == 0 {
            break;
        }
        byte_count += size;

        for b in &buffer[..size] {
            let b = *b;
            if b == b'\n' {
                line_count += 1;
            }

            if b == b' ' || b == b'\n' {
                if in_word {
                    word_count += 1;
                }
                in_word = false;
            } else {
                in_word = true;
            }
        }
    }

    Count {
        bytes: byte_count,
        lines: line_count,
        words: word_count,
    }
}
