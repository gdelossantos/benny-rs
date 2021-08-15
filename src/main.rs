use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use ansi_term::{Style, ANSIGenericString};
use ansi_term::Color::{Red, Green};
use std::path::{Path, PathBuf};

const INSERT_DIRECTIVE: &str = "==insert";

fn main() {
    let args: Vec<String> = env::args().collect();

    for file in args[1..args.len()].iter() {
        // process_template(PathBuf::from(file));
        process_template(file);
    }
}

// I really should find a good function name.
// 'process' is so overused everywhere.
fn process_template(filename: &str) {
    let path = Path::new(filename);
    println!("{}: {}", Green.paint("Processing"), bold(get_filename(path)));

    let file = File::open(&path);
    let reader = BufReader::new(file.unwrap());

    let out_fname = format!("{}.out", path. to_str().unwrap());
    let mut out_file = File::create(Path::new(&out_fname)).unwrap();

    let mut first_line = true;
    for line in reader.lines().flatten() {
        if !first_line {
            match out_file.write("\n".as_bytes()) {
                Ok(_) => {}
                Err(_) => println!("Can't write newline"),
            }
        }
        first_line = false;

        if is_insert_directive(&line) {
            let ifname = get_insert_filename(&line);
            insert_file(ifname, path, &mut out_file);
        } else {
            match out_file.write_all(line.as_bytes()) {
                Ok(_) => {}
                Err(_err) => println!("Can't write to file."),
            };
        }
    }
}

// Check if the line starts with the INSERT_DIRECTIVE
// Ignores leading whitespace
fn is_insert_directive(line: &str) -> bool {
    let s = line.trim();
    s.contains(INSERT_DIRECTIVE)
}

fn insert_file(ifname: &str, path: &Path, out_file: &mut File) {
    let ipath = get_input_path(path, ifname);

    let contents = std::fs::read_to_string(&ipath);
    if contents.is_err() {
        print_error(&["Could not open file '", ipath.to_str().unwrap(), "'"]);
        return
    }

    match out_file.write_all(contents.unwrap().as_bytes()) {
        Ok(_) => {},
        Err(why) => println!("{:?}", why),
    };

    // --------------------------------------------
    // Keeping this code here in case I need to implement indentation
    // --------------------------------------------
    // let ifile = File::open(&opath);
    // match ifile {
    //     Ok(_) => {},
    //     Err(_why) => {
    //         println!(" Could not open file: {}", ifname);
    //         return;
    //     }
    // }
    // let reader = BufReader::new(ifile.unwrap());
    // let mut first_line = true;
    // for lines in reader.lines() {
    //     if let Ok(l) = lines {
    //         if !first_line {
    //             match out_file.write("\n".as_bytes()) {
    //                 Ok(_) => {}
    //                 Err(_) => println!("Can't write newline"),
    //             }
    //         }
    //         first_line = false;
    //
    //         match out_file.write_all(l.as_bytes()) {
    //             Ok(_) => {}
    //             Err(_err) => println!("Can't write to file."),
    //         };
    //     }
    // }
}

fn get_input_path(path: &Path, ifname: &str) -> PathBuf {
    let buf = path.to_path_buf();
    let opath = match buf.parent() {
        Some(p) => p.join(ifname),
        None => PathBuf::from(ifname)
    };

    opath
}

fn get_insert_filename(line: &str) -> &str {
    line.trim()[INSERT_DIRECTIVE.len()..].trim()
}

fn bold(str: &str) -> ANSIGenericString<str> {
    Style::new().bold().paint(str)
}

fn get_filename(path: &Path) -> &str {
    path.file_name().unwrap().to_str().unwrap()
}

fn print_error(msg: &[&str]) {
    println!("{}: {}", Red.paint("error"), msg.join(""));
}

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::{BufRead, BufReader, Read, Write};
    use std::path::Path;

    #[test]
    fn test_read_file() {
        println!("Ola");
        // assert_eq!(1,2);
    }

    #[test]
    fn test_write_file() {
        let path = Path::new("test/test.out");
        let display = path.display();

        let mut f = File::create(path);
        f.unwrap().write_all("test".as_bytes());


        let lines = BufReader::new(File::open(path).unwrap()).lines();
        println!("{:?}", lines);
    }
}