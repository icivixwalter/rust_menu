// Aggiungi questa dipendenza al tuo file "Cargo.toml"
// [dependencies]
// clap = "2"

use std::{io, process::exit};

use clap::{App, Arg};

fn main() {
    pulisci_schermo(None);
    loop {
        //chiamo e stampa il menu
        menu();
        let mut scelta = String::new();

        io::stdin()
            .read_line(&mut scelta)
            .expect("Failed to read input");
        pulisci_schermo(Some(&scelta));

        // todo esegui comando
        let scelta = ElencoScelte::from(&scelta);
        //println!("{scelta}");
        esegui_scelta(&scelta);
    }
}

fn pulisci_schermo(par_scelta: Option<&str>) {
    if std::process::Command::new(clear())
        .status()
        .unwrap()
        .success()
    {
        match par_scelta {
            Some(s) => println!("eseguito {s}"),
            None => println!("scegli l'opzione da eseguire"),
        }
    }
}

//-> ritorna stringa funzione per windows o linux?
fn clear() -> String {
    if cfg!(windows) {
        "cls".to_string()
    } else {
        "clear".to_string()
    }
}

//scelta a numerico ma per ora è a stringa per 4 opzioni
enum ElencoScelte {
    EliminaFile,
    EliminaCestino,
    NessunaOperazione,
    Esci,
}
//creato i metodi per il match
impl ElencoScelte {
    fn from(par_str: &str) -> Self {
        match par_str {
            "1\n" => ElencoScelte::EliminaFile,
            "2\n" => ElencoScelte::EliminaCestino,
            "E\n" | "0\n" | "e\n" => ElencoScelte::Esci,
            _ => ElencoScelte::NessunaOperazione,
        }
    }
}
// aggiunge il parametro &= per riferimento
fn esegui_scelta(par_scelta: &ElencoScelte) {
    
    //match scelta per 3 opzioni + fuori indice
    match par_scelta {
        ElencoScelte::EliminaFile => {
            println!("Elimino il file");
        }
        ElencoScelte::EliminaCestino => {
            println!("Elimino il cestino");
        }

        ElencoScelte::Esci => {
            println!("Uscita dalla routine");
            exit(0);
        }

        ElencoScelte::NessunaOperazione => {
            println!("Nessuna operazione è impostata per questa scelta");
        }
    }

    //creare un enum per le scelte
    // per ogni scelta chiama una funzione a parte che esegue e ritorna al menu principale
}

//questi ono i menu per 3 opzioni 
fn menu() {
    println!("[1] opzione1");
    println!("[2] opzione2");
    println!("[0/E] uscita");
}