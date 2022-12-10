use std::{cell::{RefCell}, rc::Rc};

struct FileNode {
  name: Option<String>,
  size: Option<u32>,
  parent: Option<Rc<RefCell<FolderNode>>>
}

struct FolderNode {
  name: Option<String>,
  size: Option<u32>,
  folder_children: Vec<Rc<RefCell<FolderNode>>>,
  file_children: Vec<Rc<RefCell<FileNode>>>,
  parent: Option<Rc<RefCell<FolderNode>>>,
}

impl FileNode {
    pub fn new() -> FileNode {
      return FileNode { name: None, size: None, parent: None }
    }

    pub fn set_size(&mut self, new_size: Option<&str>) {
      match new_size {
          Some(sze) => {
            match sze.parse::<u32>() {
              Ok (n) => {self.size = Some(n);}
              Err(_e) => {}
            }
          }
          None => {}
      }
    }

    pub fn set_name(&mut self, new_name: Option<&str>) {
      match new_name {
          Some(nme) => {
            self.name = Some(String::from(nme));
          }
          None => {}
      }
    }
}

impl FolderNode {
    pub fn new() -> FolderNode {
      return FolderNode {
        name: None,
        size: None,
        folder_children: vec![],
        file_children: vec![],
        parent: None
      };
        
    }

    pub fn calc_folder_size(&mut self) {
      let file_children = &self.file_children;
      let folder_children = &self.folder_children;
      let mut total_size:u32 = 0;
      for file in file_children {
        match file.borrow().size {
          None => {}
          Some(s) => {
            total_size = total_size+s;
          }
        }
      }

      for folder in folder_children {
        loop {
          let mut adfa = folder.borrow_mut();
          match adfa.size {
              Some(s) => {
                total_size = total_size + s;
                break;
              }
              None => {
                adfa.calc_folder_size();
              }
          }
        }
      }

      self.size = Some(total_size);
      
    }

    pub fn get_problem_solution(&mut self) -> u32 {
      let mut totalsize:u32 = 0;
      if self.size.unwrap() <= 100000 {
        totalsize = totalsize + self.size.unwrap();
      }
      for child in &self.folder_children {
        totalsize = totalsize + child.borrow_mut().get_problem_solution();
      }

      return totalsize;
    }

    pub fn add_folder_child(&mut self, new_node: Rc<RefCell<FolderNode>>) {
      self.folder_children.push(new_node);
    }

    pub fn add_file_child(&mut self, new_node: Rc<RefCell<FileNode>>) {
      self.file_children.push(new_node);
    }

    pub fn set_name(&mut self, new_name: Option<&str>) {
      match new_name {
          Some(nme) => {
            self.name = Some(String::from(nme));
          }
          None => {}
      }
    }
}

fn main() {
  let input = include_str!("input").lines();
  //Create the filesystem root
  let root = Rc::new(RefCell::new(FolderNode::new()));
  let mut current_folder = Rc::clone(&root);
  current_folder.borrow_mut().name = Some(String::from("/"));
  //Command Parser
  for output_line in input{
    let mut split_output = output_line.split(" ");
    if split_output.clone().nth(0) == Some("$") { //input is a command
      let command = split_output.nth(1).unwrap();
      if command == "cd" {
        let new_directory_name = split_output.next().unwrap();
        println!("Moving to {}",new_directory_name);
        if new_directory_name == ".." {
          let mut new_folder : Rc<RefCell<FolderNode>> = Rc::clone(&root);
          match &current_folder.borrow_mut().parent {
            None => {}
            Some(m) => {
              new_folder = Rc::clone(&m);
            }
          }
          current_folder = new_folder
        } else {
          let mut new_folder : Rc<RefCell<FolderNode>> = Rc::clone(&root);
          for folder in &current_folder.borrow_mut().folder_children {
            if folder.borrow().name == Some(String::from(new_directory_name)) {
              new_folder = Rc::clone(&folder);
            }
          }
          current_folder = new_folder;
        }
      }
    } else if split_output.clone().nth(0) == Some("dir"){ //input is a directory
      let child = Rc::new(RefCell::new(FolderNode::new()));
      child.borrow_mut().set_name(split_output.nth(1));
      child.borrow_mut().parent = Some(Rc::clone(&current_folder));
      current_folder.borrow_mut().add_folder_child(child);
    } else if split_output.clone().nth(0).unwrap().chars().nth(0).unwrap().is_numeric() { //input is a file
      let child = Rc::new(RefCell::new(FileNode::new()));
      child.borrow_mut().set_size(split_output.next());
      child.borrow_mut().set_name(split_output.next());
      child.borrow_mut().parent = Some(Rc::clone(&current_folder));
      current_folder.borrow_mut().add_file_child(child);
    }
  }

  root.borrow_mut().calc_folder_size();
  let solution = root.borrow_mut().get_problem_solution();
  println!("{}", solution);
  
}

