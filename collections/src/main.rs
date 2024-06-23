use std::io;
use std::collections::HashMap;

fn main() {
    let median_list: [i32; 5] = [1,2,3,4,5]; 
    println!("Median: {}", find_median(&median_list));

    let mode_list: [i32; 6] = [1,2,3,4,4,5]; 
    println!("Mode {}", find_mode(&mode_list));

    let sentence = vec!["Little", "piggy", "stinks", "of", "poop"];
    println!("Pigs Latin: {:?}", translate_to_pig_latin(&sentence));

    add_employee();
}

fn find_median(list_of_ints: &[i32]) -> i32 {
    let mid = list_of_ints.len() / 2;
    list_of_ints[mid]
}

fn find_mode(list_of_ints: &[i32]) -> i32 {
    let mut entry_to_frequency: HashMap<i32, i32> = HashMap::new();

    for &number in list_of_ints {
        let count = entry_to_frequency.entry(number).or_insert(0);
        *count += 1;
    }

    *entry_to_frequency.iter()
                       .max_by(|a, b| a.1.cmp(&b.1))
                       .map(|(k, _v)| k)
                       .unwrap()
}

fn translate_to_pig_latin(sentence: &Vec<&str>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for &word in sentence {
        if let Some(first_char) = word.chars().next() {
            if first_char == 'a' || first_char == 'e' || first_char == 'i' || first_char == 'o' || first_char == 'u' {
                let new_word = format!("{}-hay", word);
                result.push(new_word);
            } else {
                let new_word = format!("{}-{}ay", &word[1..], first_char);
                result.push(new_word);
            }
        } 
    }
    result
}

fn add_employee() {
    let mut department_to_employees: HashMap<String, Vec<String>> = HashMap::new(); 
    department_to_employees.insert(String::from("sales"), Vec::new());
    department_to_employees.insert(String::from("finance"), Vec::new());
    
    loop {
        println!("Enter an employee name to one of the departments, or enter a department to retrieve employees. Departments are sales or finance.");

        let mut input = String::new();

        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

        let trimmed = input.trim().to_lowercase();
        let mut words = trimmed.split_whitespace().map(String::from);

        if let Some(first_word) = words.next() {
            if first_word == "add" {
                if let Some(name) = words.next() {
                    //skip "to"
                    words.next();
                    if let Some(dep) = words.next() {
                        if department_to_employees.contains_key(&dep.to_lowercase()) {
                            println!("Added {} to {}", &name, &dep);
                            department_to_employees.get_mut(&dep.to_lowercase())
                                                   .unwrap()
                                                   .push(name);
                                                
                        } else {
                            println!("Employee can only be added to sales or finance. Found {}.", &dep);
                        }
                    } else {
                        println!("Enter either sales or finance. You need to type e.g., Add john to sales");
                    }
                } else {
                    println!("WRONG! You need to type e.g., Add jane to finance");
                }
                continue
            } else if first_word == "sales" {
                    println!("Sales: {:?}", department_to_employees.get("sales").unwrap());
            } else if first_word == "finance" {
                    println!("Finance: {:?}", department_to_employees.get("finance").unwrap());
            }
        }
    }
}
