use std::io;
use rand::Rng;
use std::fs;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};
extern crate walkdir;
use walkdir::WalkDir;

fn main() {
    let path = "saved_group_lists";
    match fs::create_dir_all(path) {
        Err(_) => panic!("Something went wrong. Make sure you are running this program as Administrator."),
        _ => {},
    }
    let saved_list_count = WalkDir::new(path).into_iter().count();

    if saved_list_count > 1 {
        println!("Here are your saved lists:");
        let paths = fs::read_dir(path).unwrap();

        for cpath in paths {
            println!("{}", cpath.unwrap().path().display())
        }
        println!("Would you like to use one of your saved lists? [y/n]");
        let mut use_saved_list: String;
        loop {
            let mut guess = String::new();
                io::stdin()
                    .read_line(&mut guess)
                    .expect("Failed to read line");
    
            let guess: String = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {println!("Oops! Something went wrong. Remember to only type the number, no spaces."); continue;},
            };
            use_saved_list = match guess.trim().parse::<String>() {
                Ok(c) => {if c == "y" || c == "n" {c} else {println!("Oops! Please try again. Remember to type 'smaller' or 'larger' exactly!"); continue;}},
                Err(_) => {println!("Oops! Please try again."); continue;},
            };
            if use_saved_list == "n" {
                not_using_saved_list();
            }
            break;
        }
        if use_saved_list == "y" {
            println!("Please type the name of the list you would like to use. \nNo need to type the 'saved_group_lists/', just the name of the unique list.");
            loop {
                let mut input = String::new();
                io::stdin().read_line(&mut input).expect("Failed to read line");
                let input: String = match input.trim().parse() {
                    Ok(c) => c,
                    Err(_) => {println!("Oops! Please try again."); continue;},
                };
                let mut open_path:String = String::from("saved_group_lists/");
                open_path.push_str(&input);
                let save_file = File::open(open_path);
                if let Err(_) = save_file {
                    println!("Oops! That list does not exist. Please try again.");
                    continue;
                }
                let save_file = save_file.unwrap();
                let buffered = BufReader::new(save_file);
                let mut people: Vec<String> = Vec::new();
                for line in buffered.lines() {
                    people.push(line.unwrap());
                }
            println!("How many people in each group?");
            let mut number: u32;
            loop {
                let mut guess = String::new();
                    io::stdin()
                        .read_line(&mut guess)
                        .expect("Failed to read line");

                    let guess: u32 = match guess.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {println!("Oops! Something went wrong. Remember to only type the number, no spaces."); continue;},
                    };
                    number = guess;
                    break;
                }
            let (modulo, big_or_small) = get_modulus_options(&people, &number);
            create_groups(people, big_or_small, modulo, number);
            quit_when_ready();
            break;
            }
        }

    } else {
        not_using_saved_list();
    }
}

fn not_using_saved_list() {
    let mut list_file: File;
    let mut save_next_list = String::new();
    println!("Would you like to save the list you are about to create? \nPlease type 'yes' or 'no' exactly");
        loop {
            let mut guess = String::new();
                io::stdin()
                    .read_line(&mut guess)
                    .expect("Failed to read line");
    
                let guess: String = match guess.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {println!("Oops! Something went wrong. Remember to only type the number, no spaces."); continue;},
                };
                save_next_list = match guess.trim().parse::<String>() {
                    Ok(c) => {if c == "yes" || c == "no" {c} else {println!("Oops! Please try again. Remember to type 'smaller' or 'larger' exactly!"); continue;}},
                    Err(_) => {println!("Oops! Please try again."); continue;},
                };
                break;
        }
        if save_next_list == "yes" {
            println!("What would you like to call this list?");
            let mut guess = String::new();
                io::stdin()
                    .read_line(&mut guess)
                    .expect("Failed to read line");
            let mut list_path: String = String::from("saved_group_lists/");
            let other = guess.trim().parse::<String>().unwrap();
            list_path.push_str(&other);
            list_file = File::create(list_path).unwrap();
            let (mut people, mut number) = create_new_list();
            let people_clone = people.clone();
            for person in people_clone {
                writeln!(list_file, "{}", person);
            }
            let (modulo, big_or_small) = get_modulus_options(&people, &number);
            create_groups(people, big_or_small, modulo, number);
        } else {
            let (mut people, mut number) = create_new_list();
            let (modulo, big_or_small) = get_modulus_options(&people, &number);
            create_groups(people, big_or_small, modulo, number);
            quit_when_ready();
        }
}

fn create_groups(mut people: Vec<String>, big_or_small: String, modulo: usize, number: u32) {
    while people.len() > {if big_or_small == "smaller" {modulo} else {modulo + (modulo as usize * number as usize)}} {
        let mut current_group: Vec<String> = Vec::new();
        for _i in 0..number {
            let current_person = rand::thread_rng().gen_range(0, people.len());
            current_group.push(people[current_person].clone());
            people.remove(current_person);
        }
        println!("{:?}", current_group);
        println!("");
    }
    if big_or_small == "smaller" {
        println!("{:?}", people);
        println!("Here are your randomized groups! If this was helpful, \nplease consider asking C Null (creationincbyc@gmail.com like Creation Inc. By C) to make more useful programs.");
    } else {
        while people.len() > 0 {
            let mut current_group: Vec<String> = Vec::new();
            for _i in 0..(number as usize + 1) {
                let current_person = rand::thread_rng().gen_range(0, people.len());
                current_group.push(people[current_person].clone());
                people.remove(current_person);
            }
            println!("{:?}", current_group);
            println!(""); 
        }
        println!("Here are your randomized groups! If this was helpful, \nplease consider asking C Null (creationincbyc@gmail.com like Creation Inc. By C) to make more useful programs.");
    }
}

fn create_new_list() -> (Vec<String>, u32) {
    println!("How many people in each group?");
    let mut number: u32;
    loop {
        let mut guess = String::new();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line");

            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {println!("Oops! Something went wrong. Remember to only type the number, no spaces."); continue;},
            };
            number = guess;
            break;
        }

    let mut people: Vec<String> = Vec::new();
    println!("Type the name of each person and then press enter.\nWhen you are done, type 'end' exactly (lowercase) and press enter.");
    println!("If at any time you need to start over type 'start over' EXACTLY, paying attention to making it lowercase.");
    println!("If at any time you would like to see the current list, type 'show list' exactly.");
    loop {
        let mut current_person = String::new();
        io::stdin()
            .read_line(&mut current_person)
            .expect("Failed to read line.");

        let _current_person = match current_person.trim().parse::<String>() {
            Ok(p) => {if p == "end" {break;} else if p == "start over" {people = Vec::<String>::new(); println!("Okay, clearing the list. Please start over."); continue;} else if p == "show list" {println!("Here is the list: {:?}", people); continue;}; people.push(p.to_string());},
            Err(_) => {println!("Oops! Please try that one again."); continue;},
        };
    }
    return (people, number);
}

fn get_modulus_options(people: &Vec<String>, number: &u32) -> (usize, String) {
    let modulo = people.len() % *number as usize;
    let mut big_or_small = String::new();
    if modulo > (people.len() / *number as usize) {
        big_or_small = String::from("smaller");
        return (modulo, big_or_small);
    }
    loop {
        let mut choice = String::new();
        if modulo != 0 {
            println!("This number of people does not fit exactly into the number of people in each group. Would you like {} groups with one more person or one smaller group of {} people? Please type 'smaller' or 'larger'", modulo, modulo);
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        big_or_small = match choice.trim().parse::<String>() {
            Ok(c) => {if c == "smaller" {c} else if c == "larger" {c} else {println!("Oops! Please try again. Remember to type 'smaller' or 'larger' exactly!"); continue;}},
            Err(_) => {println!("Oops! Please try again."); continue;},
        };
        }
        break;
    }
    return (modulo, big_or_small);
}

fn quit_when_ready() {
    println!("Press enter to exit");
    let mut placeholder = String::new();
    io::stdin()
            .read_line(&mut placeholder)
            .expect("Failed to read line.");
}
