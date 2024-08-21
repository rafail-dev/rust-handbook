use std::collections::HashMap;
use std::io;

enum Result {
    IsQuit,
    Message(Vec<String>),
}

#[allow(dead_code)]
fn main() {
    let mut db: HashMap<String, String> = HashMap::new();

    loop {
        let command = io::stdin()
            .lines()
            .next()
            .expect("Failed to read line")
            .expect("Failed to parse line");

        let s = command.split_whitespace().collect::<Vec<&str>>();

        let result: Result = match s.as_slice() {
            ["quit"] => Result::IsQuit,
            another => Result::Message(match another {
                ["all"] => {
                    let mut vec: Vec<(&String, &String)> = db.iter().map(|(e, d)| (d, e)).collect();

                    vec.sort();

                    let result = vec.iter().map(|(d, e)| format!("{}\t{}", d, e));

                    std::iter::once(format!("All Employees"))
                        .chain(result)
                        .collect::<Vec<String>>()
                }
                [department] => {
                    let mut vec: Vec<(&String, &String)> = db
                        .iter()
                        .filter(|(_, d)| d == department)
                        .map(|(e, d)| (d, e))
                        .collect();

                    vec.sort();

                    let result = vec.iter().map(|(d, e)| format!("{}\t{}", d, e));

                    std::iter::once(format!("Employees from {}", department))
                        .chain(result)
                        .collect::<Vec<String>>()
                }

                ["add", employee, "to", department] => {
                    db.insert(employee.to_string(), department.to_string());

                    vec![format!("{} added to {}", employee, department)]
                }

                ["fire", employee] => match db.remove(&employee.to_string()) {
                    Some(department) => {
                        vec![format!("{} fired from {}", employee, department)]
                    }
                    None => {
                        vec![format!("Employee not found")]
                    }
                },
                _ => vec![command],
            }),
        };

        match result {
            Result::IsQuit => {
                println!("Bye, Bye");
                break;
            }
            Result::Message(s) => s
                .iter()
                .chain(std::iter::once(&String::new()))
                .for_each(|s| println!("{}", s)),
        }
    }
}
