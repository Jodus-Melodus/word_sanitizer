use std::{
    io::{self, Write, BufReader, BufRead},
    path::PathBuf,
    fs::{File, OpenOptions},
    env,
};

fn readln(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush output");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim_end().to_string()
}

fn read_file(path: &PathBuf) -> String {
    let file = File::open(path).unwrap_or_else(|_| panic!("Failed to open file {:?}", path));
    let content: Vec<String> = BufReader::new(&file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    content.join("\n")
}

fn write_file(content: &str, path: &PathBuf) {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)  // Use write(true) instead of append(true)
        .open(path)
        .unwrap_or_else(|e| panic!("Failed to open file {:?}: {}", path, e));

    file.write_all(content.as_bytes())
        .unwrap_or_else(|e| panic!("Failed to write to file {:?}: {}", path, e));

    file.flush().unwrap_or_else(|e| panic!("Failed to flush file {:?}: {}", path, e));
}

fn sanitize(content: &str, chars_to_exclude: &str, word_length:usize) -> String {
    let mut res: Vec<String> = Vec::new();
    let words = content.split('\n').collect::<Vec<&str>>();
    let char_vec: Vec<char> = chars_to_exclude.chars().collect();
    for word in words {
        if word.contains(char_vec.as_slice()) && word.len() <= word_length {
            continue;
        } else {
            res.push(word.to_owned());
        }
    }
    res.join("\n")
}

fn get_current_directory() -> PathBuf {
    env::current_dir().unwrap_or_else(|_| PathBuf::from("."))
}

fn main() {
    println!("Welcome to word Sanitizer!");

    loop {
        let command = readln("Enter file path > ");
        
        if command == "exit" || command == "quit" {
            break;
        } else {
            let file_path = command;
            let characters = readln("Enter characters to remove > ");
            let lenght = readln("Enter length of allowed words > ").parse::<usize>().unwrap_or(100);

            let mut path = get_current_directory();
            path.push(PathBuf::from(file_path));
            let content = read_file(&path);
            let result = sanitize(content.as_str(), &characters, lenght);
            write_file(&result, &path);
            println!("File has been updated!");
        }
    }
    
}
