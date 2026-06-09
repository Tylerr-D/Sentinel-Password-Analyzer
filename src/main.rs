//hello

use rand::Rng;

const LOWERCASE: &str = "qwertyuiopasdfghjklzxcvbnm";
const UPPERCASE: &str = "QWERTYUIOPASDFGHJKLZXCVBNM";
const NUMBERS: &str = "0123456789";
const SPECIALS: &str = "!@#$%^&*";


fn generate_password() {
    println!("Generating password...");

    let characters = format!(
        "{}{}{}{}",
        LOWERCASE,
        UPPERCASE,
        NUMBERS,
        SPECIALS
    );

    let mut gen_password = String::new();

    let mut rng = rand::rng();

    for _twd_is_a_great_show in 0..12 {
        let index = rng.random_range(0..characters.len());

        let ch = characters.chars().nth(index).unwrap();

        gen_password.push(ch);
    }

    println!("Generated Password: {}", gen_password);

    loop {
    println!("\nGenerate another password? (y/n)");

    let mut choice = String::new();

    std::io::stdin()
        .read_line(&mut choice)
        .expect("Failed to read input");

    if choice.trim() == "y" {
        generate_password();
        break;
    }

    else if choice.trim() == "n" {
        break;
    }

    else {
        println!("Please enter y or n");
    }
}
}
fn check_length(password: &str) -> bool {

let length  = password.len();

if length >= 8 {
println!("✓ Length requirement passed");
true
}

else {
println!("✗ Password must be at least 8 characters");
false
}
}

fn check_uppercase(password: &str) -> bool {

    let mut found_uppercase = false;

    for me in password.chars(){
        if me.is_uppercase(){
            found_uppercase = true;
            break;
        }
    }

    if found_uppercase {
                println!("✓ Contains uppercase letter");
                true

    }

else{
            println!("✗ No uppercase letters found");
            false

}
}


fn check_number(password:&str) -> bool {

    let mut number_found = false;

    for you in password.chars(){
           if you.is_ascii_digit() {
            number_found = true;
            break;
    }
}

if number_found {
    println!("✓ You have got numbers");
    true
}

else {
 println!("✗ You dont have numbers");
 false

}
}

fn check_special_char(password:&str) -> bool {

    let mut special_char_found = false;

    for someone in password.chars(){
           if !someone.is_alphanumeric() {
            special_char_found = true;
            break;
    }
}

if special_char_found {
    println!("✓ You have got special characters");
    true
}

else {
 println!("✗ You dont have special characters");
 false

}
}


fn main(){

    //this is big brain moment right here

    'main_loop: loop {
        println!();
println!("=================================
    Sentinel Password Analyzer
=================================");
println!();

println!("1. Analyze Password");
println!("2. Generate Password");
println!("3. Exit");

println!("Choose an option:");

let mut choice = String::new();

std::io::stdin()
    .read_line(&mut choice)
    .expect("failed to read input");


if choice.trim() == "1"{

    println!();

    println!("enter your password");


let mut user_password = String::new();

std::io::stdin()
.read_line(&mut user_password)
.expect("failed to read input");

println!();

let mut score = 0;

let password = user_password.trim();

if check_length(password){
  score+=1;
}

if check_number(password){
  score+=1;
}

if check_special_char(password){
      score+=1;
}

if check_uppercase(password){
      score+=1;
}

println!();

println!("\n===== PASSWORD REPORT =====");
println!();
println!("Password Length: {}", password.len());
println!("Security level: {}/4",score);

if score <= 2 {
    println!("Password Strength: weak");
}

if score == 3 {
    println!("Password Strength: mid");
}

if score == 4 {
    println!("Password Strength: strong");
}

println!();

loop {
println!("Analyze another password? (y/n)");

let mut yes_no = String::new();

std::io::stdin()
.read_line(&mut yes_no)
.expect("failed to read input");

if yes_no.trim() == "n" {
        println!("bye");
    break;
}

else if yes_no.trim() == "y" {
    continue 'main_loop;
}

else {
        println!("enter valid input")
    
}
}
    }



else if choice.trim() == "2" {
    generate_password();
}


else if choice.trim() == "3"{
    break;
}

    

else {
    println!("choose a valid option broski");
}
    
    }

}

