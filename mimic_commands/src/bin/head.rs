use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::process::exit;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    let max_size = 10;
    if args.len() <= 1 {
        print_head(io::stdin(), max_size);
        return;
    }

    if args.len() == 2 {
        let path = &args[1];
        print_file_head(path, max_size);
        return;
    }

    let paths = &args[1..];
    let mut error_occurred = false;
    for (i, path) in paths.iter().enumerate() {
        println!("==> {} <==", path);
        let is_succeed = print_file_head(path, max_size);
        if !is_succeed {
            error_occurred = true;
        }

        if i != paths.len() {
            println!()
        }
    }

    if error_occurred {
        exit(1);
    }
}

fn print_file_head(path: &str, max_count: usize) -> bool {
    match File::open(path) {
        Ok(f) => {
            // print_head(f, max_count);
            print_head_buf(f, max_count);
            true
        }
        Err(_) => {
            io::stderr()
                .write_all(format!("head: {}: No such file or directory\n", path).as_ref())
                .ok();
            false
        }
    }
}

fn print_head<T: Read>(mut reader: T, max_count: usize) {
    let mut stdout = io::stdout();
    let mut buffer = [0; 1024 * 4];
    let mut line_count = 0;
    loop {
        let size = reader.read(&mut buffer[..]).expect("failed to read");
        if size == 0 {
            break;
        }

        let mut print_size = 0;
        for b in &buffer[..size] {
            print_size += 1;

            if *b == b'\n' {
                line_count += 1;
                if line_count == max_count {
                    break;
                }
            }
        }

        stdout
            .write_all(&buffer[..print_size])
            .expect("failed to write");

        if line_count == max_count {
            break;
        }
    }
}

/// Using BufReader and BufReader can simplify code while suppressing system calls.
fn print_head_buf<T: Read>(reader: T, max_count: usize) {
    let mut reader = BufReader::new(reader);
    let mut writer = BufWriter::new(io::stdout());
    let mut one_byte = [0; 1];

    let mut line_count = 1;
    while line_count <= max_count {
        let size = reader.read(&mut one_byte).expect("failed to read");
        if size == 0 {
            break;
        }

        let b = one_byte[0];
        if b == b'\n' {
            line_count += 1;
        }

        writer.write_all(&one_byte).expect("failed to write");
    }
    writer.flush().expect("failed to write");
}
