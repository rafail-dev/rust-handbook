use std::collections::HashMap;
use std::io;

#[allow(dead_code)]
fn main() {
    let mut db: HashMap<String, String> = HashMap::new();

    loop {
        let command = io::stdin()
            .lines()
            .next()
            .expect("Failed to read line")
            .expect("Failed to parse line");

        let s = command.split_whitespace().map(|s| s).collect::<Vec<&str>>();

        match s.as_slice() {
            ["quit"] => {
                println!("Bye, bye");
                break;
            }
            ["all"] => {
                let mut vec: Vec<(&String, &String)> = db.iter().map(|(e, d)| (d, e)).collect();

                vec.sort();

                vec.iter().for_each(|(d, e)| println!("{}\t{}", d, e));
            }
            [department] => {
                let mut vec: Vec<(&String, &String)> = db
                    .iter()
                    .filter(|(_, d)| d == department)
                    .map(|(e, d)| (d, e))
                    .collect();

                vec.sort();

                vec.iter().for_each(|(d, e)| println!("{}\t{}", d, e));
            }

            ["Add", employee, "to", department] => {
                db.insert(employee.to_string(), department.to_string());

                println!("{} added to {}", employee, department);
            }

            ["Fire", employee] => match db.remove(*employee) {
                Some(department) => {
                    println!("{} fired from P{}", employee, department)
                }
                None => {
                    println!("Employee not found")
                }
            },

            _ => {
                println!("{:?}", s);
            }
        }
        println!();
    }
}
