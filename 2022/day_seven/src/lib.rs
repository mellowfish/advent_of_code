use std::borrow::Borrow;
use std::ops::{Deref, DerefMut};

struct FileEntry {
    name: String,
    size: usize
}

impl FileEntry {
    fn new(input: &str) -> Self {
        let parts : Vec<&str> = input.split(" ").collect();
        if parts.len() != 2 {
            panic!("Malformed file line: {input}")
        }

        Self { name: parts[1].to_string(), size: parts[0].parse().unwrap() }
    }
}

struct DirectoryEntry {
    parent: Option<String>,
    name: String,
    entries: Vec<FileSystemEntry>
}

impl DirectoryEntry {
    fn build_path(parent: &String, child: &String) -> String {
        format!("{}/{}", parent, child)
    }

    fn extract_parent_path(path: &String) -> String {
        let mut segments : Vec<&str> = path.split("/").collect();
        segments.pop();
        segments.join("/")
    }

    fn root() -> Self {
        Self { parent: None, name: String::from("/"), entries: vec![] }
    }

    fn new(parent: &str, name: &str) -> Self {
        Self { parent: Some(parent.to_string()), name: name.to_string(), entries: vec![] }
    }

    fn path(&self) -> String {
        match &self.parent {
            None => { self.name.clone() },
            Some(path) => { Self::build_path(path, &self.name) }
        }
    }

    fn add(&mut self, entry: FileSystemEntry) {
        self.entries.push(entry)
    }

    fn add_directory(&mut self, name: &str) -> Box<&DirectoryEntry> {
        self.entries.push(FileSystemEntry::DirectoryEntry(DirectoryEntry::new(self.path().as_str(), name)));

        match self.entries.last().unwrap() {
            FileSystemEntry::DirectoryEntry(directory_entry) => {
                return Box::from(directory_entry);
            },
            _ => { panic!("Unexpected type") }
        }
    }

    fn traverse(&mut self, path: &str, add_missing_directories: bool) -> &DirectoryEntry {
        if path.is_empty() || path.eq("/") {
            return self;
        }

        let mut parts = path.split("/");
        let current_segment = parts.next().unwrap();
        let rest = parts.collect::<Vec<&str>>().join("/");

        for entry in self.entries.iter() {
            match entry {
                FileSystemEntry::DirectoryEntry(mut directory_entry) => {
                    if directory_entry.name.eq(current_segment) {
                        return directory_entry.traverse(rest.as_str(), add_missing_directories);
                    }
                },
                FileSystemEntry::FileEntry(_) => {}
            }
        }

        if add_missing_directories {
            let rtn : &DirectoryEntry;
            unsafe {
                let boxed_entry = self.add_directory(current_segment);
                let directory_entry = boxed_entry.deref().deref();
                rtn = directory_entry.traverse(rest.as_str(), add_missing_directories);
            }
            return rtn;
        }

        panic!("No directory at path: {path}")
    }
}

enum FileSystemEntry {
    FileEntry(FileEntry),
    DirectoryEntry(DirectoryEntry)
}

struct FileSystem {
    root: DirectoryEntry
}

impl FileSystem {
    fn new() -> Self {
        Self { root: DirectoryEntry::root() }
    }

    fn from_bash_session(input: &str) -> Self {
        let mut session = BashSession::new();
        session.run_commands(input);

        session.file_system
    }

    fn add_directory(&mut self, parent_directory: &str, new_directory: &str) {
        self.root.traverse(
            DirectoryEntry::build_path(&parent_directory.to_string(), &new_directory.to_string()).as_str(),
            true
        );
    }

    fn get_directory(&mut self, path: &str) -> &DirectoryEntry {
        self.root.traverse(path, false)
    }

    fn get_parent_path(&mut self, path: &str) -> String {
        self.get_directory(DirectoryEntry::extract_parent_path(&path.to_string()).as_str()).path()
    }
}

struct BashSession {
    file_system: FileSystem,
    current_directory: String
}

impl BashSession {
    fn new() -> Self {
        Self { file_system: FileSystem::new(), current_directory: String::from("/") }
    }

    fn run_commands(&mut self, input: &str) {
        let mut lines = input.lines();

        while let Some(line) = lines.next() {
            let parts : Vec<&str> = line.split(" ").collect();
            match parts[0] {
                "$" => {
                    match parts[1] {
                        "ls" => {
                            // TODO
                        },
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
                    // TODO: file listing
                }
            }
        }
    }

    fn add_directory(&mut self, directory: &str) {
        self.file_system.add_directory(&self.current_directory, directory)
    }

    fn change_directory(&mut self, directory: &str) {
        self.current_directory =
            match directory {
                ".." => {
                    self.file_system.get_parent_path(&self.current_directory)
                },
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
        assert_eq!(file_system.smaller_folders().iter().map(DirectoryEntry::size), 95437);
    }
}
