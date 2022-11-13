use std::io;
use std::fs;
use std::fs::File;
use std::io::{prelude::*, BufReader};
use lopdf::Document;
use chrono::Datelike;

static FILENAME: &str = "dipendenti.txt";
static OUTPUTDIR: &str = "output";
static PDFNAME: &str = "bustepaga.pdf";

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

        let dip_num = dipendenti.len();
        println!("Presenti: {} dipendenti", dip_num);
        if dip_num == 0 {
            println!("Non ci sono dipendenti nel file: {}, aggiungi i nomi per continuare", FILENAME);
            continue;
        }
        
        // for value in &dipendenti {
        //     println!("{}", value);
        // }

        fs::create_dir_all(OUTPUTDIR).unwrap();

        load_pdf(dipendenti);
    }
}

fn print_menu() {
    println!("1) Dividi buste paga per dipendente");
    println!("2) Esci");
    println!("Inserici il valore desiderato e premi INVIO");
}

fn read_file(dip: &mut Vec<String>) {
    //debug: 
    //println!("{}", std::env::current_dir().unwrap().display());
    let file_result = File::open(FILENAME);

    let greeting_file = match file_result {
        Ok(file) => file,
        Err(_) => panic!("Non riesco a trovare il file dipendenti: {}", FILENAME),
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

fn get_date() -> String {
    let current_date = chrono::Utc::now();
    let year = current_date.year();
    let month = current_date.month();
    return format!("{}{}{}{}", "_", month, "_", year);
}

fn load_pdf(dip: Vec<String>) {
    let doc = match Document::load(PDFNAME) {
        Ok(_doc) => _doc,
        Err(_) => panic!("Non riesco a trovare il file buste paga: {}", PDFNAME)
    };
    let count = doc.get_pages().len();
    println!("Il PDF contiene {} buste paga", count);

    if count > dip.len() {
        println!("Ci sono piú buste paga che dipendenti!");
        return;
    } 
    if count < dip.len() {
        println!("Ci sono piú dipendenti che buste paga!");
        return;
    }

    let data = get_date();

    for n in 1..=count {
        let mut doc_copy = doc.clone();
        let mut file_name = String::from(&dip[n-1]);
        file_name.push_str(&data);
        file_name.push_str(".pdf");

        let mut del_pages: Vec<u32> = Vec::new();
        for x in 1..=count {
            if x == n {
                continue;
            }
            del_pages.push(x.try_into().unwrap());
        }
        doc_copy.delete_pages(&del_pages);
        let path = format!("{}{}{}", OUTPUTDIR, "/", file_name);
        println!("Creazione della busta paga numero {}, nome: {}", n, file_name);
        doc_copy.save(path).unwrap();
    }
}