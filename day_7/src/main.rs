use regex::Regex;
use std::collections::HashMap;
use std::fs;

#[derive(Debug)]
struct ElfDir {
    name: String,
    file_list: Vec<ElfFile>,
    dir_list: Vec<ElfDir>,
}

#[derive(Debug)]
struct ElfFile {
    size: u32,
}

#[derive(Debug)]
struct FileSystem {
    current_dir: String,
    curr_path: Vec<String>,
    dir_list: HashMap<String, ElfDir>,
}

impl ElfFile {
    fn new(size: u32) -> ElfFile {
        return ElfFile { size };
    }

    fn parse(line: String) -> Result<ElfFile, String> {
        let mut result = line.split_whitespace();
        let size = match result.next() {
            Some(char) => {
                if char == "$" || char == "dir" {
                    return Err(String::from("Not a file"));
                }
                char.parse().unwrap()
            }
            None => return Err(String::from("No string")),
        };

        return Ok(ElfFile::new(size));
    }
}

impl ElfDir {
    fn new(name: String) -> ElfDir {
        return ElfDir {
            name,
            file_list: Vec::new(),
            dir_list: Vec::new(),
        };
    }

    fn add_file(&mut self, file: ElfFile) {
        self.file_list.push(file);
    }
    fn add_dir(&mut self, dir: ElfDir) {
        self.dir_list.push(dir);
    }

    fn size(&self, filesys: &FileSystem) -> u32 {
        let mut total: u32 = 0;
        for file in self.file_list.iter() {
            total += file.size;
        }
        for dir in self.dir_list.iter() {
            let dir_obj = filesys.dir_list.get(&dir.name).unwrap();
            total += dir_obj.size(filesys);
        }

        return total;
    }
}

impl FileSystem {
    fn new() -> FileSystem {
        let mut sys = FileSystem {
            current_dir: String::from("/"),
            curr_path: Vec::new(),
            dir_list: HashMap::new(),
        };
        sys.curr_path.push(sys.current_dir.to_string());
        sys.dir_list.insert(
            sys.write_curr_path(),
            ElfDir::new(sys.current_dir.to_string()),
        );
        return sys;
    }
    fn write_curr_path(&self) -> String {
        return self.curr_path.join("-");
    }
}

fn parse_chdir_line(mut filesys: FileSystem, line: String) -> FileSystem {
    let dir: Vec<&str> = line.split_whitespace().collect();
    let dir = dir[2];
    if dir == String::from("/") {
        filesys.curr_path = Vec::new();
    } else if dir == String::from("..") {
        filesys.curr_path.pop();
        filesys.current_dir = filesys.write_curr_path();
        return filesys;
    }
    filesys.curr_path.push(dir.to_string());
    filesys.dir_list.insert(
        filesys.write_curr_path(),
        ElfDir::new(filesys.write_curr_path()),
    );
    filesys.current_dir = filesys.write_curr_path();

    return filesys;
}

fn parse_file_line(mut filesys: FileSystem, line: String) -> FileSystem {
    let curr_dir = filesys
        .dir_list
        .get_mut(&filesys.write_curr_path())
        .unwrap();
    curr_dir.add_file(ElfFile::parse(line).unwrap());
    return filesys;
}

fn parse_dir_line(mut filesys: FileSystem, line: String) -> FileSystem {
    let dir: Vec<&str> = line.split_whitespace().collect();
    let dir = dir[1];
    let new_dir_str = format!("{}-{}", filesys.write_curr_path(), dir);
    filesys.dir_list.insert(
        new_dir_str.to_string(),
        ElfDir::new(new_dir_str.to_string()),
    );
    let curr_dir = filesys
        .dir_list
        .get_mut(&filesys.write_curr_path())
        .unwrap();
    curr_dir.add_dir(ElfDir::new(new_dir_str));

    return filesys;
}

fn part1(lines: &String) {
    let mut filesys = FileSystem::new();
    let file_regex = Regex::new(r"^(\d+) (\S+)").unwrap();
    for line in lines.lines() {
        if line.contains("$ cd") {
            filesys = parse_chdir_line(filesys, line.to_string());
        } else if line.contains("dir") {
            filesys = parse_dir_line(filesys, line.to_string());
        } else if file_regex.is_match(line) {
            filesys = parse_file_line(filesys, line.to_string());
        }
    }
    let mut deletable_size = 0;
    for (_, dir) in &filesys.dir_list {
        let dirsize = dir.size(&filesys);
        if dirsize <= 100000 {
            deletable_size += dirsize;
        }
    }
    println!("{:?}", deletable_size);
}
fn part2(lines: &String) {
    let mut filesys = FileSystem::new();
    let file_regex = Regex::new(r"^(\d+) (\S+)").unwrap();
    for line in lines.lines() {
        if line.contains("$ cd") {
            filesys = parse_chdir_line(filesys, line.to_string());
        } else if line.contains("dir") {
            filesys = parse_dir_line(filesys, line.to_string());
        } else if file_regex.is_match(line) {
            filesys = parse_file_line(filesys, line.to_string());
        }
    }
    let root_space = &filesys.dir_list.get("/").unwrap().size(&filesys);
    println!("We have this much space: {}", 70000000 - root_space);
    let space_needed = 30000000 - (70000000 - root_space);
    println!("We need this much space: {}", space_needed);

    let mut deletable_size = 70000000;
    let mut deletable_dir = String::from("None");
    for (_, dir) in &filesys.dir_list {
        let dirsize = dir.size(&filesys);
        if dirsize > space_needed && dirsize < deletable_size {
            deletable_size = dirsize;
            deletable_dir = dir.name.to_string();
        }
    }
    println!("Smallest deletable directory size: {}", deletable_size);
    println!("Smallest deletable directory name: {}", deletable_dir);
}

fn read_puzzle_input(path: &str) -> String {
    fs::read_to_string(path).expect("Could not get the puzzle input")
}

fn main() {
    let input = read_puzzle_input("./input");
    part1(&input);
    part2(&input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_line_parse() {
        let line = String::from("62596 h.lst");
        let file = ElfFile::parse(line).unwrap();
        assert_eq!(file.size, 62596);
    }

    #[test]
    fn test_dir_line_parse() {
        let line = String::from("dir d");
        let dir = ElfDir::parse(line);
        assert_eq!(dir.unwrap().name, "d");
    }
    #[test]
    fn test_add_file() {
        let filesys = FileSystem::new();
        let line = String::from("dir d");
        let mut dir = ElfDir::parse(line).unwrap();
        let line = String::from("62596 h.lst");
        let file = ElfFile::parse(line).unwrap();
        dir.add_file(file);
        assert_eq!(dir.name, "d");
        assert_eq!(dir.size(&filesys), 62596);
    }

    #[test]
    fn test_add_two_file() {
        let filesys = FileSystem::new();
        let line = String::from("dir d");
        let mut dir = ElfDir::parse(line).unwrap();
        let line = String::from("62596 h.lst");
        let file = ElfFile::parse(line).unwrap();
        dir.add_file(file);
        let line = String::from("62596 h.lst");
        let file = ElfFile::parse(line).unwrap();
        assert_eq!(dir.name, "d");
        dir.add_file(file);
        assert_eq!(dir.size(&filesys), 62596 * 2);
    }

    #[test]
    fn test_parse_chdir_line() {
        let mut filesys = FileSystem::new();
        let line = String::from("$ cd /");
        filesys = parse_chdir_line(filesys, line);
        assert_eq!(filesys.write_curr_path(), "/");
        let line = String::from("$ cd e");
        filesys = parse_chdir_line(filesys, line);
        let line = String::from("$ cd f");
        filesys = parse_chdir_line(filesys, line);
        assert_eq!(filesys.write_curr_path(), "/-e-f");
        let line = String::from("$ cd ..");
        filesys = parse_chdir_line(filesys, line);
        assert_eq!(filesys.write_curr_path(), "/-e");
    }

    #[test]
    fn test_parse_addfile_line() {
        let mut filesys = FileSystem::new();
        let line = String::from("$ cd e");
        filesys = parse_chdir_line(filesys, line);
        let line = String::from("62596 h.lst");
        filesys = parse_file_line(filesys, line);
        println!("{:?}", filesys);
    }

    #[test]
    fn test_parse_dir_line() {
        let mut filesys = FileSystem::new();
        let line = String::from("dir e");
        filesys = parse_dir_line(filesys, line);
        let line = String::from("$ cd e");
        filesys = parse_chdir_line(filesys, line);
        let line = String::from("dir f");
        filesys = parse_dir_line(filesys, line);
        println!("{:?}", filesys);
    }
}
