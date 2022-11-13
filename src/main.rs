use std::io;

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

        println!("Valore: {input_value}");
    }
}

fn print_menu() {
    println!("1) Test");
    println!("2) Esci");
    println!("Inserici il valore desiderato e premi INVIO");
}
