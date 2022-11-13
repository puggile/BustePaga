use std::io;
use std::fs::File;
use std::io::{prelude::*, BufReader};

static FILENAME: &str = "testFile.txt";

fn main() {
    loop {
        // stampa menu
        print_menu();

        // leggo da std input
        let mut input_value = String::new();
        io::stdin()
        .read_line(&mut input_value)
        .expect("Errore nella lettura!");

        // parsing del valore in intero
        let input_value: i32 = match input_value
        .trim()
        .parse::<i32>() {
            Ok(n) => n,
            Err(_) => {
                println!("Inserisci un numero valido");
                continue;
            }
        };

        if input_value < 1 || input_value > 2 {
            println!("Inserisci un numero compreso tra 1 e 2");
            continue;
        }

        if input_value == 2 {
            println!("Uscita dal programma");
            break;
        }

        let mut dipendenti = Vec::new();
        read_file(&mut dipendenti);

        println!("Presenti: {} dipendenti", dipendenti.len());
        
        for value in &dipendenti {
            println!("{}", value);
        }
    }
}

fn print_menu() {
    println!("1) Test");
    println!("2) Esci");
    println!("Inserici il valore desiderato e premi INVIO");
}

fn read_file(dip: &mut Vec<String>) {
    //debug: 
    //println!("{}", std::env::current_dir().unwrap().display());
    let file_result = File::open(FILENAME);

    let greeting_file = match file_result {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let reader = BufReader::new(greeting_file);
    for line in reader.lines() {
        match line {
            Ok(s) => {
                //debug: 
                //println!("{}", s);
                dip.push(s);
            }
            Err(_) => println!("Errore linea")
        }
    }
}
