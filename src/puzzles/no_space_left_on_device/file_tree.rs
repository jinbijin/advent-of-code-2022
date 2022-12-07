use std::collections::HashMap;

use super::command_line::{ChangeDirectoryTarget, CommandLine, DirectoryItem};

#[derive(Debug, Clone)]
pub struct File {
    pub size: usize,
    pub name: String,
}

#[derive(Debug, Clone)]
pub struct Directory {
    pub directories: HashMap<String, Directory>,
    pub files: Vec<File>,
}

impl Directory {
    fn new() -> Directory {
        Directory {
            directories: HashMap::new(),
            files: Vec::new(),
        }
    }

    pub fn get_size(&self) -> usize {
        let file_sum = self.files.iter().map(|file| file.size).sum::<usize>();
        let directory_sum = self
            .directories
            .values()
            .map(|dir| dir.get_size())
            .sum::<usize>();
        file_sum + directory_sum
    }

    fn get_directory_mut(&mut self, path: &Vec<String>) -> Option<&mut Directory> {
        let mut current = self;
        for name in path.iter() {
            let next = current.directories.get_mut(name)?;
            current = next;
        }
        Some(current)
    }

    // TODO Really inefficient...
    pub fn get_directories_smaller_than(&self, bound: usize) -> Vec<(&Directory, usize)> {
        let mut directory_sizes = self
            .directories
            .values()
            .flat_map(|dir| dir.get_directories_smaller_than(bound))
            .collect::<Vec<(&Directory, usize)>>();
        let directory_size = self.get_size();
        if directory_size <= bound {
            directory_sizes.push((self, directory_size));
        }
        directory_sizes
    }

    // TODO Really inefficient...
    pub fn get_directories_larger_than(&self, bound: usize) -> Vec<(&Directory, usize)> {
        let mut directory_sizes = self
            .directories
            .values()
            .flat_map(|dir| dir.get_directories_larger_than(bound))
            .collect::<Vec<(&Directory, usize)>>();
        let directory_size = self.get_size();
        if directory_size >= bound {
            directory_sizes.push((self, directory_size));
        }
        directory_sizes
    }
}

pub struct CommandLineIterator<T>
where
    T: Iterator<Item = CommandLine>,
{
    source: T,
    file_system: Directory,
    current_path: Vec<String>, // TODO I'd really like a reference to a directory, and not some string path...
}

impl<T> CommandLineIterator<T>
where
    T: Iterator<Item = CommandLine>,
{
    fn process_command(&mut self, command_line: CommandLine) -> () {
        match command_line {
            CommandLine::List => {}
            CommandLine::ChangeDirectory(target) => self.process_directory_change(target),
            CommandLine::Item(directory_item) => self.process_item(directory_item),
        }
    }

    fn process_directory_change(&mut self, target: ChangeDirectoryTarget) -> () {
        match target {
            ChangeDirectoryTarget::Into(name) => {
                self.current_path.push(name);
            }
            ChangeDirectoryTarget::Out => {
                if self.current_path.len() > 0 {
                    self.current_path.pop();
                }
            }
            ChangeDirectoryTarget::Root => {
                while self.current_path.len() > 0 {
                    self.current_path.pop();
                }
            }
        }
    }

    fn process_item(&mut self, directory_item: DirectoryItem) -> () {
        if let Some(current) = self.file_system.get_directory_mut(&self.current_path) {
            match directory_item {
                DirectoryItem::Directory { name } => {
                    current.directories.insert(name, Directory::new());
                }
                DirectoryItem::File { size, name } => {
                    current.files.push(File { size, name });
                }
            }
        }
    }
}

impl<T> Iterator for CommandLineIterator<T>
where
    T: Iterator<Item = CommandLine>,
{
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        match self.source.next() {
            None => None,
            Some(command_line) => Some(self.process_command(command_line)),
        }
    }
}

impl FromIterator<CommandLine> for Directory {
    fn from_iter<T: IntoIterator<Item = CommandLine>>(iter: T) -> Self {
        let iter = iter.into_iter();
        let mut iter = CommandLineIterator {
            source: iter,
            file_system: Directory::new(),
            current_path: Vec::new(),
        };

        while let Some(()) = iter.next() {}

        iter.file_system
    }
}
