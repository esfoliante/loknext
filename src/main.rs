use arboard::Clipboard;
use colored::Colorize;
use dotenv::dotenv;
use magic_crypt::{new_magic_crypt, MagicCrypt256, MagicCryptTrait};
use rand::distributions::{Alphanumeric, DistString};

use std::collections::HashMap;
use std::fs::{read_to_string, File};
use std::io::{self, Write};
use std::path::Path;

use loknext::kv_map;

static FILENAME: &str = ".loknext";

fn main() -> () {
    /*
     * This line simply loads all of our .env variables
     * (if the file is not yet created, check .env.example)
     */
    dotenv().ok();

    /*
     * This token takes care of encrypting your data.
     * If this token is equal to other's token, be careful
     */
    let crypt_token: String = std::env::var("CRYPT_TOKEN").expect("CRYPT_TOKEN must be set.");

    let magic_crypt: MagicCrypt256 = new_magic_crypt!(crypt_token, 256);
    let mut passwords_file: Result<File, std::io::Error> = get_passwords_file();

    clear_screen();

    println!("LOKNEXT - Terminal Password Manager");
    println!("Type {} if you need any.", "'help'".bold().green());
    println!();

    loop {
        let user_input: String = read_string("".to_string());

        parse_instruction(
            &user_input.clone(),
            magic_crypt.clone(),
            passwords_file.as_mut().unwrap(),
        );
    }
}

fn read_string(message: String) -> String {
    let stdin = io::stdin();
    let mut user_input = String::new();

    if message != "" {
        print!("{} >> ", message);
    } else {
        print!(">> ");
    }

    io::stdout().flush().expect("Failed to flush string.");

    let _ = stdin
        .read_line(&mut user_input)
        .expect("Failed to read string.");

    return user_input.trim().replace("\n", "");
}

/*
* This function will take care of handling the passwords file.
* If the file is not created it will be created and returned
* in order to be used. Otherwise, it will open the
* passwords with the appending option on
*/
fn get_passwords_file() -> Result<File, std::io::Error> {
    let path: &Path = Path::new(FILENAME);

    if path.exists() {
        return File::options().append(true).open(FILENAME);
    }

    return File::create(FILENAME);
}

/*
* This function, that definitely isn't scary, has the purpose of clearing
* the screen using special characters and, after that, setting our
* cursor to line 1, column 1
*/
fn clear_screen() -> () {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn parse_instruction(
    instruction: &str,
    magic_crypt: MagicCrypt256,
    passwords_file: &mut File,
) -> () {
    let formatted_instruction: &str = &instruction.to_uppercase();
    let mut clipboard = Clipboard::new().unwrap();

    let instruction_vector: Vec<&str> = formatted_instruction.split(" ").collect::<Vec<&str>>();

    if formatted_instruction.contains("GP") {
        let mut length: i8 = -1;

        if instruction_vector.len() > 1 {
            length = instruction_vector[1].parse().unwrap();
        }

        let password: String = generate_password(length);

        println!("{} => {}", "Generated password", password.bold().red());
        println!(
            "Your password was copied to your {}",
            "Clipboard".bold().green()
        );

        /*
        	* After generating the password and showing it to the user,
        	* we want to add it to the clipboard so the user can
        	* just continue with its' life
        	*/
        clipboard.set_text(password).unwrap();

        return;
    }

    if formatted_instruction.contains("CR") {
        let data: Vec<String> = get_records();
        let limit = data.len() - 1;

        /*
         * If the lenth of the instruction_vector is 1 then it means
         * that the user didn't pass any addicional arguments
         */
        if instruction_vector.len() == 1 {
            println!("Services:\n");

            for i in (0..limit).step_by(4) {
                println!("{}", data[i].bold().green());
            }

            return;
        }

        let service_name: &str = instruction_vector[1];

        for i in (0..limit).step_by(4) {
            if service_name == data[i].to_uppercase() {
                let password: String = decrypt_password(magic_crypt, data[i + 2].to_string());

                println!("Email / username >> {}", data[i + 1].bold().red());
                println!("Password >> {}\n", password.bold().red());

                println!("Your password was copied to your clipboard.");

                clipboard.set_text(password).unwrap();

                return;
            }
        }

        println!("Service '{}' not found.", service_name.bold().green());

        return;
    }

    match formatted_instruction {
        "CLEAR" => clear_screen(),
        "HELP" => help_command(),
        "EXIT" => std::process::exit(0x0100),
        "AR" => add_record(magic_crypt, passwords_file),
        _ => println!("Instruction '{}' does not exist.", formatted_instruction),
    }
}

fn help_command() -> () {
    let commands: Vec<HashMap<&str, &str>> = vec![
        kv_map!("name" => "exit", "description" => "Exits the program."),
        kv_map!("name" => "clear", "description" => "Clears the console screen."),
        kv_map!("name" => "help", "description" => "Shows all the commands and their descriptions."),
        kv_map!("name" => "ar", "description" => "Stands for 'add record' and adds a new password."),
        kv_map!("name" => "cr", "description" => "Stands for 'check record' and shows all the services registered. Add a parameter after with the name of a service and shows the specific record."),
        kv_map!("name" => "gp", "description" => "Stands for 'generate password' and generates a strong password. Add a parameter with an integer greater than 1 to set a specific length (default = 12)."),
    ];

    for command in commands {
        println!(
            "{} - {}",
            command.get("name").unwrap().bold().green(),
            command.get("description").unwrap()
        );
    }
}

/*
* This function generates a random set of characters and
* returns it as a string, with the ability of receiving
* the length of the password via parameter.
*/
fn generate_password(length: i8) -> String {
    let password_length: i8;

    /*
    	* This condition makes sure that we handle the password size
    	* properly, allowing us not to pass a parameter
    	*/
    match length {
        -1 => password_length = 12,
        _ => password_length = length,
    }

    return Alphanumeric
        .sample_string(&mut rand::thread_rng(), password_length.try_into().unwrap());
}

fn add_record(magic_crypt: MagicCrypt256, passwords_file: &mut File) -> () {
    let mut service_name: String;
    let mut email: String;

    loop {
        service_name = read_string("Service name".to_string());

        if service_name != "" {
            break;
        }
    }

    loop {
        email = read_string("Email / Username".to_string());

        if email != "" {
            break;
        }
    }

    let mut password: String = read_string("Password (empty to generate one)".to_string());

    if password == "" {
        password = generate_password(12);

        println!("Generated password >> {}", password.bold().red());
    }

    let encrypted_password: String = encrypt_password(magic_crypt, password) + "\n";

    service_name += "\n";
    email += "\n";

    /*
     * Write all the data to the file
     */
    let _ = passwords_file.write_all(service_name.as_bytes());
    let _ = passwords_file.write_all(email.as_bytes());
    let _ = passwords_file.write_all(encrypted_password.as_bytes());
    let _ = passwords_file.write_all("---end---\n".as_bytes());
}

/*
* This function will get all the data
* from the passwords file
*/
fn get_records() -> Vec<String> {
    let data: String = read_to_string(FILENAME).unwrap();

    return data.lines().map(str::to_string).collect();
}

/*
* With the power of science (and some hardcore math that I don't understand)
* we're going to take a simple string and encrypt it
*/
fn encrypt_password(magic_crypt: MagicCrypt256, password: String) -> String {
    return magic_crypt.encrypt_str_to_base64(password);
}

/*
* Now, for the decryption process we're going to use this function down here.
* Once again, I barely understand the decryption process
*/
fn decrypt_password(magic_crypt: MagicCrypt256, crypted_password: String) -> String {
    let decrypted_password: String = magic_crypt
        .decrypt_base64_to_string(&crypted_password)
        .unwrap();

    return decrypted_password;
}
