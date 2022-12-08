use std::str::Lines;

enum Command<'a> {
    ChangeDirectory(&'a str),
    ListCurrentDirectory,
}

impl<'a> Command<'a> {
    fn try_parse(text: &'a str) -> Option<Self> {
        let mut text = text.split_whitespace();
        if let Some(prefix) = text.next() {
            if prefix != "$" {
                return None;
            }
        }

        match text.next() {
            Some(word) => match word {
                "ls" => Some(Command::ListCurrentDirectory),
                "cd" => Some(Command::ChangeDirectory(text.next()?)),
                _ => None,
            },
            _ => None,
        }
    }
}

enum File {
    File { name: String, size: u32 },
    Folder { name: String, files: Box<Vec<File>> },
}

impl File {
    fn try_parse(text: &str) -> Option<Self> {
        let mut text = text.split_whitespace();
        match text.next() {
            Some(word) => match word {
                "dir" => Some(File::Folder {
                    name: String::from(text.next()?),
                    files: Box::new(Vec::new()),
                }),
                word => {
                    // If the word is not "dir", then `word` here is the file size
                    if let Ok(size) = word.parse::<u32>() {
                        return Some(File::File {
                            name: String::from(text.next()?),
                            size,
                        });
                    }
                    None
                }
            },
            _ => None,
        }
    }

    fn get_name(&self) -> &str {
        match self {
            File::File { name, .. } => name,
            File::Folder { name, .. } => name,
        }
    }

    fn get_size(&self) -> u32 {
        match self {
            File::File { size, .. } => *size,
            File::Folder { files, .. } => files.iter().map(|x| x.get_size()).sum(),
        }
    }
}

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

fn build_directory(lines: &mut Lines, context_folder: &mut File) -> () {
    if let File::Folder { files, .. } = context_folder {
        while let Some(line) = lines.next() {
            // We'll first assume we're handling a file
            if let Some(file) = File::try_parse(line) {
                files.push(file);
                continue;
            }

            // If we failed to parse a file, we'll try to parse a command
            if let Some(cmd) = Command::try_parse(line) {
                match cmd {
                    Command::ChangeDirectory(dir) => {
                        if dir == ".." {
                            return;
                        }

                        if let Some(index) = files.iter().position(|file| file.get_name() == dir) {
                            build_directory(lines, &mut files[index]);
                        }
                    }
                    Command::ListCurrentDirectory => continue,
                }
            }
        }
    }
}

fn total_size_of_directories_less_than_100000(directory: &File) -> u32 {
    let mut sum = 0;

    if let File::Folder { files, .. } = directory {
        if directory.get_size() < 100000 {
            sum += directory.get_size()
        }

        for file in files.iter() {
            sum += match file {
                File::Folder { .. } => total_size_of_directories_less_than_100000(&file),
                _ => 0,
            };
        }
    }

    sum
}

fn smallest_folder_size_greater_than(size: u32, directory: &File) -> Option<u32> {
    let mut possible_folder_sizes = Vec::new();

    if let File::Folder { files, .. } = directory {
        if directory.get_size() > size {
            possible_folder_sizes.push(Some(directory.get_size()));
        }

        for file in files.iter() {
            match file {
                File::Folder { .. } => {
                    possible_folder_sizes.push(smallest_folder_size_greater_than(size, &file))
                }
                _ => continue,
            };
        }
    }

    if possible_folder_sizes.len() == 0 {
        return None;
    }

    possible_folder_sizes.iter().flatten().min().copied()
}

fn part_one(input: &str) -> u32 {
    let mut lines = input.lines();

    match lines.next() {
        Some("$ cd /") => {
            let mut root_directory = &mut File::Folder {
                name: String::from("/"),
                files: Box::new(Vec::new()),
            };
            build_directory(&mut lines, &mut root_directory);
            total_size_of_directories_less_than_100000(root_directory)
        }
        _ => panic!("Failed to 'cd' into '/' first"),
    }
}

fn part_two(input: &str) -> u32 {
    const FILESYSTEM_SIZE: u32 = 70000000;
    const UPDATE_SIZE: u32 = 30000000;
    let mut lines = input.lines();

    match lines.next() {
        Some("$ cd /") => {
            let mut root_directory = &mut File::Folder {
                name: String::from("/"),
                files: Box::new(Vec::new()),
            };
            build_directory(&mut lines, &mut root_directory);

            let current_free_space_available = FILESYSTEM_SIZE - root_directory.get_size();
            let space_needed = UPDATE_SIZE - current_free_space_available;
            match smallest_folder_size_greater_than(space_needed, &root_directory) {
                Some(answer) => answer,
                _ => panic!("Something went wrong"),
            }
        }
        _ => panic!("Failed to 'cd' into '/' first"),
    }
}
