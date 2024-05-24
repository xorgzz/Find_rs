use std::env;
use std::fs;
use std::io;
use std::io::prelude::*;
use std::path::Path;


fn color(r: i32, g: i32, b: i32, text: &str) -> String {
    return format!("\x1B[38;2;{};{};{}m{}\x1B[0m", r, g, b, text);
}

fn dir_crawler(base_path: &str, filter: &str) {
    let paths = fs::read_dir(base_path).unwrap();

    for path in paths {
        let new_path = path.unwrap().path();
        let file_dir = new_path.to_str().unwrap();
        let new_path = Path::new(file_dir);
        let name = new_path.file_name().unwrap();
        
        if !new_path.is_file() { // Directory
            dir_crawler(file_dir, filter);
            
        } else if name.to_str().unwrap().contains(filter) { // File
            let splitter: &str;
            let splited_dir: core::str::Split<'_, &str>;
            if file_dir.contains("/") {
                splitter = "/";
                splited_dir = file_dir.split("/");
            } else {
                splitter = "\\";
                splited_dir = file_dir.split("\\");
            }
            let len_splited_dir = splited_dir.clone().collect::<Vec<_>>().len();

            let splited_name = name.to_str().unwrap().split(filter);
            let len_splited_name = splited_name.clone().collect::<Vec<_>>().len();  

            let mut iter = 0;

            print!("    ");
            for slice in splited_dir {
                if iter == 0 {
                    iter += 1;
                    continue;
                }
                if iter >= len_splited_dir-1 {
                    iter+=1;
                    continue;
                }
                print!("{}{}", color(100, 255, 255, slice), color(255, 255, 100, splitter));
                iter += 1;
            }

            iter = 0;

            for slice in splited_name {
                print!("{}", color(100, 255, 255, slice));
                if iter < len_splited_name-1 {
                    print!("{}", color(100, 255, 100, filter));
                }
                iter+=1;
            }
            println!();
        
        }
    }
}

fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    println!("\n\n{} {} {}", color(255, 255, 100, "Press"), color(200, 60, 60, "ENTER"), color(255, 255, 100, "to exit..."));
    stdout.flush().unwrap();

    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 2 {
        println!("{}", color(218, 112, 214, "Find_rs found: \n"));
        dir_crawler(".", &args[1])
    }

    pause();
}
