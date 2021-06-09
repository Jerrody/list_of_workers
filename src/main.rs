use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{stdin, BufRead, Write},
    process::exit,
};

fn main() -> io::Result<()> {
    let mut list_of_employers: HashMap<String, Vec<String>> = HashMap::new();

    for lines in stdin().lock().lines() {
        match Command::process_command(&lines.unwrap()) {
            Some(Command::Add {
                department,
                employer,
            }) => {
                list_of_employers
                    .entry(department)
                    .or_default()
                    .push(employer);
            },
            Some(Command::DeleteDepartment(department)) => {
                match list_of_employers.remove_entry(&department) {
                    Some((department, employers)) => {
                        println!("These employers were unbind from {}", department);
                        for employer in employers {
                            println!("{}\n", employer);
                        }
                    },
                    None => continue,
                }
            },
            Some(Command::DeleteEmployer {
                department,
                employer,
            }) => match list_of_employers.get_mut(&department) {
                Some(employers) => {
                    let index = employers.iter().position(|name| *name == employer).unwrap();
                    employers.remove(index);
                },
                None => continue,
            },
            Some(Command::List(department)) => match list_of_employers.get(&department) {
                Some(employers) => {
                    for employer in employers {
                        println!("{}", employer);
                    }
                },
                None => {
                    println!("Can't find in database this department!");
                },
            },
            Some(Command::All) => {
                for (department, employers) in &list_of_employers {
                    let mut employers: Vec<String> = employers.clone();
                    employers.sort();

                    println!("{}", department);
                    for employer in employers {
                        println!("\t{}", employer);
                    }
                }
            },
            Some(Command::Quit) => {
                let mut file = OpenOptions::new()
                    .write(true)
                    .open("list_of_employers.txt")
                    .unwrap_or(File::create("list_of_employers.txt")?);

                for (department, employers) in &list_of_employers {
                    let mut employers: Vec<String> = employers.clone();
                    employers.sort();

                    write!(file, "{}\n", department)?;
                    for employer in employers {
                        write!(file, "\t{}\n", employer)?;
                    }
                }

                exit(0);
            },
            None => {
                println!("No such command!");
                continue;
            },
        }
    }

    Ok(())
}

enum Command {
    Add {
        department: String,
        employer: String,
    },
    DeleteDepartment(String),
    DeleteEmployer {
        department: String,
        employer: String,
    },
    List(String),
    All,
    Quit,
}

impl Command {
    fn process_command(input: &String) -> Option<Self> {
        let word: Vec<&str> = input.split_whitespace().collect();

        match word.as_slice() {
            ["Add", employer, "to", department] => Some(Self::Add {
                department: department.to_string(),
                employer: employer.to_string(),
            }),
            ["Remove department", department] => {
                Some(Self::DeleteDepartment(department.to_string()))
            }
            ["Remove employer", employer, "from", department] => Some(Self::DeleteEmployer {
                department: department.to_string(),
                employer: employer.to_string(),
            }),
            ["List", department] => Some(Self::List(department.to_string())),
            ["All"] => Some(Self::All),
            ["Quit"] => Some(Self::Quit),
            _ => None,
        }
    }
}

// ######################################################################################################################
// Implementation without an Option
// ######################################################################################################################
// fn main() -> Result<()> {
//     let mut list_of_workers: HashMap<String, Vec<String>> = HashMap::new();

//     for lines in stdin().lock().lines() {
//         match Command::process_command(&lines.unwrap()) {
//             Command::Add {
//                 department,
//                 employer,
//             } => {
//                 list_of_workers
//                     .entry(department)
//                     .or_default()
//                     .push(employer);
//             },
//             Command::DeleteDepartment(department) => {
//                 match list_of_workers.remove_entry(&department) {
//                     Some((department, employers)) => {
//                         println!("These employers were unbind from {}", department);
//                         for employer in employers {
//                             println!("{}\n", employer);
//                         }
//                     },
//                     None => continue,
//                 }
//             },
//             Command::DeleteEmployer {
//                 department,
//                 employer,
//             } => match list_of_workers.get_mut(&department) {
//                 Some(employers) => {
//                     let index = employers.iter().position(|name| *name == employer).unwrap();
//                     employers.remove(index);
//                 },
//                 None => continue,
//             },
//             Command::List(department) => match list_of_workers.get(&department) {
//                 Some(employers) => {
//                     for employer in employers {
//                         println!("{}", employer);
//                     }
//                 }
//                 None => {
//                     println!("Can't find in database this department!");
//                 }
//             },
//             Command::All => {
//                 for (department, employers) in &list_of_workers {
//                     let mut employers: Vec<String> = employers.clone();
//                     employers.sort();

//                     println!("{}", department);
//                     for employer in employers {
//                         println!("\t{}", employer);
//                     }
//                 }
//             },
//             Command::Quit => {
//                 let mut file = OpenOptions::new()
//                     .write(true)
//                     .open("list_of_employers.txt")
//                     .unwrap_or(File::create("list_of_employers.txt")?);

//                 for (department, employers) in &list_of_workers {
//                     let mut employers: Vec<String> = employers.clone();
//                     employers.sort();

//                     writeln!(file, "{}", department)?;
//                     for employer in employers {
//                         writeln!(file, "\t{}", employer)?;
//                     }
//                 }

//                 exit(0);
//             },
//             Command::Threshold => println!("Type Command!"),
//         }
//     }

//     Ok(())
// }

// enum Command {
//     Add {
//         department: String,
//         employer: String,
//     },
//     DeleteDepartment(String),
//     DeleteEmployer {
//         department: String,
//         employer: String,
//     },
//     List(String),
//     All,
//     Quit,
//     Threshold,
// }

// impl Command {
//     fn process_command(input: &str) -> Self {
//         let word: Vec<&str> = input.split_whitespace().collect();

//         match word.as_slice() {
//             ["Add", employer, "to", department] => Self::Add {
//                 department: department.to_string(),
//                 employer: employer.to_string(),
//             },
//             ["Remove department", department] => Self::DeleteDepartment(department.to_string()),
//             ["Remove employer", employer, "from", department] => Self::DeleteEmployer {
//                 department: department.to_string(),
//                 employer: employer.to_string(),
//             },
//             ["List", department] => Self::List(department.to_string()),
//             ["All"] => Self::All,
//             ["Quit"] => Self::Quit,
//             _ => Self::Threshold,
//         }
//     }
// }
// ######################################################################################################################