use regex::Regex;
use std::collections::HashMap;
use std::io;
fn main() {
    println!("Welcome to company!");
    let mut database: Vec<HashMap<String, String>> = Vec::new();
    loop {
        println!("Please select ");
        println!("[0] Preload employees to database");
        println!("[1] Add employee to database");
        println!("[2] Fetch employees in a department");
        println!("[3] List employees (alphabetical)");
        println!("[4] Exit");
        let mut text_interface_input = String::new();
        io::stdin()
            .read_line(&mut text_interface_input)
            .expect("Failed to read line");

        match text_interface_input.trim().parse() {
            Ok(num) => match num {
                0 => preload(&mut database),
                1 => add_wizard(&mut database),
                2 => display_employee_list_in_dept(&database),
                3 => display_employee_list(&database),
                4 => return,
                _ => continue,
            },
            Err(c) => {
                println!("{} is an invalid input. Only (1-3) allowed", c);
                continue;
            }
        };
    }
}
fn preload(mut database: &mut Vec<HashMap<String, String>>) {
    let input_parse_regex = Regex::new(r"add ([a-zA-Z]+) to ([a-zA-Z]+)").unwrap();
    let feed = "add luke to tech, add han to sales, add chewie to sales, add palpatine to hr, add vader to tech, add lando to sales, add jabba to hr, add yoda to hr";
    for cap in input_parse_regex.captures_iter(feed) {
        add(&cap[1], &cap[2], &mut database);
    }
    println!("Preload successful!");
}
fn add_wizard(mut database: &mut Vec<HashMap<String, String>>) {
    let mut input = String::new();
    println!("**************************Adding***************************");
    println!("Enter an employee in the format : ");
    println!("Add Name to Department");
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.to_lowercase();
    let input_parse_regex = Regex::new(r"add ([a-zA-Z]+) to ([a-zA-Z]+)").unwrap();
    for cap in input_parse_regex.captures_iter(&input[..]) {
        add(&cap[1], &cap[2], &mut database);
        println!(
            "{name} has been added to {dept}",
            name = &cap[1],
            dept = &cap[2]
        );
    }
}
fn add(name: &str, department: &str, database: &mut Vec<HashMap<String, String>>) {
    let mut employee: HashMap<String, String> = HashMap::new();
    employee.insert(String::from("name"), name.to_string());
    employee.insert(String::from("department"), department.to_string());
    database.push(employee);
}
fn get_department_list(database: &Vec<HashMap<String, String>>) -> Vec<String> {
    //TODO : Since I suck at closures right now, I need to update this to get actual list of departments using map or something
    // let dept_list: Vec<String> = database.iter().map(|x| x.get("department")).collect();
    vec![
        String::from("sales"),
        String::from("hr"),
        String::from("tech"),
    ]
}
fn display_employee_list_in_dept(database: &Vec<HashMap<String, String>>) {
    let mut input = String::new();
    let database_list: Vec<String> = get_department_list(&database);
    println!("**************************Employees in Department***************************");
    println!("Enter name of department from {:?}", database_list);
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    let input = input.trim().to_lowercase();

    if database_list.iter().any(|x| x == &input) {
        print!("List of employees are : ");
        for employee in database {
            if employee.get("department").unwrap() == &input {
                print!("{},", employee.get("name").unwrap().to_string());
            }
        }
        println!("");
    } else {
        println!("Please enter valid department. You have entered {}", input);
    }
}

fn display_employee_list(database: &Vec<HashMap<String, String>>) {
    println!("**************************All Employees***************************");
    let mut name_sorted_database = database.clone();
    name_sorted_database.sort_by(|a, b| a.get("name").unwrap().cmp(b.get("name").unwrap()));

    println!("Employees are : ");
    for employee in name_sorted_database {
        println!(
            "{name} | {department}",
            name = employee.get("name").unwrap(),
            department = employee.get("department").unwrap()
        );
    }
}

#[test]
fn test_get_department_list() {
    let mut database: Vec<HashMap<String, String>> = Vec::new();
    let names = vec![
        String::from("kenny"),
        String::from("joe"),
        String::from("travis"),
    ];
    let department_list = vec![
        String::from("sales"),
        String::from("hr"),
        String::from("tech"),
    ];
    for (name, dept) in names.into_iter().zip(department_list.clone().into_iter()) {
        let mut employee: HashMap<String, String> = HashMap::new();
        employee.insert(String::from("name"), name);
        employee.insert(String::from("department"), dept);
        database.push(employee);
    }
    assert_eq!(&department_list, &get_department_list(&database));
}
