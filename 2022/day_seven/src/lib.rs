use std::collections::HashMap;

#[derive(Debug)]
struct FileEntry {
    #[allow(dead_code)]
    name: String,
    size: usize
}

impl FileEntry {
    fn new(name: &str, size: usize) -> Self {
        Self { name: name.to_string(), size }
    }
}

#[derive(Debug)]
struct DirectoryEntry {
    parent: Option<String>,
    name: String,
    children: Vec<String>,
    files: Vec<FileEntry>
}

impl DirectoryEntry {
    fn build_path(parent: &String, child: &String) -> String {
        if parent == "/" {
            format!("/{child}")
        } else {
            format!("{parent}/{child}")
        }
    }

    fn extract_parent_path(path: &String) -> String {
        let mut segments : Vec<&str> = path.split("/").collect();
        segments.pop();
        segments.join("/")
    }

    fn root() -> Self {
        Self { parent: None, name: String::from("/"), children: vec![], files: vec![] }
    }

    fn new(parent: &str, name: &str) -> Self {
        Self { parent: Some(parent.to_string()), name: name.to_string(), children: vec![], files: vec![] }
    }

    fn path(&self) -> String {
        match &self.parent {
            None => { self.name.clone() },
            Some(path) => { Self::build_path(path, &self.name) }
        }
    }

    fn total_file_size(&self) -> usize {
        self.files.iter().map(|file| file.size).sum()
    }

    fn add_file(&mut self, entry: FileEntry) {
        self.files.push(entry)
    }

    fn add_directory(&mut self, name: &str) -> String {
        let name_string = name.to_string();

        self.children.push(name_string.clone());

        Self::build_path(&self.path(), &name_string)
    }
}

struct FileSystem {
    root: HashMap<String, DirectoryEntry>
}

impl FileSystem {
    fn new() -> Self {
        Self {
            root: HashMap::from([(String::from("/"), DirectoryEntry::root())])
        }
    }

    fn from_bash_session(input: &str) -> Self {
        let mut session = BashSession::new();
        session.run_commands(input);

        session.file_system
    }

    fn add_directory(&mut self, parent_directory: &str, new_directory: &str) {
        let new_path = self.root.get_mut(parent_directory).unwrap().add_directory(new_directory);

        self.root.insert(new_path, DirectoryEntry::new(parent_directory, new_directory));
    }

    fn add_file(&mut self, parent_directory: &str, file_name: &str, file_size: usize) {
        self.root.get_mut(parent_directory).unwrap().add_file(FileEntry::new(file_name, file_size))
    }

    fn get_directory(&self, path: &str) -> &DirectoryEntry {
        self.root.get(path).unwrap()
    }

    fn directory_size(&self, path: &str) -> usize {
        let path_string = path.to_string();
        let directory = self.get_directory(path);
        let mut size = directory.total_file_size();

        for child_dir in directory.children.iter() {
            size += self.directory_size(DirectoryEntry::build_path(&path_string, &child_dir).as_str())
        }

        size
    }

    fn directory_sizes(&self) -> Vec<usize> {
        let mut directories = self.root.values().map(|directory| self.directory_size(directory.path().as_str())).collect::<Vec<usize>>();

        directories.sort();
        directories.reverse();

        directories
    }

    fn smaller_folders(&self) -> Vec<usize> {
        self.directory_sizes().iter()
            .filter(|size| **size < 100000)
            .map(|size| *size)
            .collect()
    }

    fn used_space(&self) -> usize {
        self.directory_size("/")
    }

    fn free_space(&self) -> usize {
        70000000 - self.used_space()
    }

    fn space_to_free_up_for_update(&self) -> usize {
        30000000 - self.free_space()
    }

    fn smallest_folder_size_to_delete(&self) -> usize {
        let target_size = self.space_to_free_up_for_update();
        let mut last_directory_size = 0;

        for &directory_size in self.directory_sizes().iter() {
            if directory_size < target_size {
                return last_directory_size;
            }
            last_directory_size = directory_size
        }

        0
    }
}

struct BashSession {
    file_system: FileSystem,
    current_directory: String
}

impl BashSession {
    fn new() -> Self {
        Self {
            file_system: FileSystem::new(),
            current_directory: String::from("/")
        }
    }

    fn run_commands(&mut self, input: &str) {
        let mut lines = input.lines();

        while let Some(line) = lines.next() {
            let parts : Vec<&str> = line.split(" ").collect();
            match parts[0] {
                "$" => {
                    match parts[1] {
                        "ls" => {},
                        "cd" => {
                            self.change_directory(parts[2]);
                        },
                        _ => panic!("unknown command: {line}")
                    }
                },
                "dir" => {
                    self.add_directory(parts[1]);
                },
                _ => {
                    self.add_file(parts[1], parts[0].parse::<usize>().unwrap());
                }
            }
        }
    }

    fn add_directory(&mut self, directory: &str) {
        self.file_system.add_directory(&self.current_directory, directory)
    }

    fn add_file(&mut self, file_name: &str, file_size: usize) {
        self.file_system.add_file(&self.current_directory, file_name, file_size)
    }

    fn change_directory(&mut self, directory: &str) {
        self.current_directory =
            match directory {
                ".." => {
                    DirectoryEntry::extract_parent_path(&self.current_directory)
                },
                "/"=> { String::from("/") }
                _ => {
                    DirectoryEntry::build_path(&self.current_directory, &directory.to_string())
                }
            };
    }
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    #[test]
    fn part_one_examples() {
        let file_system = FileSystem::from_bash_session(fs::read_to_string("example_input.txt").unwrap().as_str());
        assert_eq!(file_system.smaller_folders().iter().sum::<usize>(), 95437);
    }

    #[test]
    fn part_one() {
        let file_system = FileSystem::from_bash_session(fs::read_to_string("input.txt").unwrap().as_str());
        assert_eq!(file_system.smaller_folders().iter().sum::<usize>(), 1555642);
    }

    #[test]
    fn part_two_example() {
        let file_system = FileSystem::from_bash_session(fs::read_to_string("example_input.txt").unwrap().as_str());

        assert_eq!(file_system.smallest_folder_size_to_delete(), 24933642);
    }

    #[test]
    fn part_two() {
        let file_system = FileSystem::from_bash_session(fs::read_to_string("input.txt").unwrap().as_str());

        assert_eq!(file_system.smallest_folder_size_to_delete(), 5974547);
    }
}
