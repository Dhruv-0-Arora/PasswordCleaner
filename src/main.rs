use std::{io, error::Error, process};

#[derive(Debug, serde::Deserialize)]
struct Entry {
    folder: String,
    favorite: bool,
    type_: String,
    name: String,
    notes: String,
    fields: String,
    reprompt: String,
    login_url: String,
    login_username: String,
    login_password: String,
    login_totp: String,
}

fn fetch_entries() -> Result<(), Box<dyn Error>> { 
    let mut rdr = csv::Reader::from_reader(io::stdin());
    for result in rdr.deserialize() {
        let record: Record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

//fn write_file() {
//
//}

fn main() {
    println!("Please ensure you are in the same directory as your password csv file. Please then input the name of your csv file ending with '*.csv'");
    let mut file_name = String::new();
    let _ = io::stdin().read_line(&mut file_name);
    if let Err(err) = fetch_entries(file_name) {
        println!("error running example: {}", err);
        process::exit(1);
    }
}

