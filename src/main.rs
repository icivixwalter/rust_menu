// Aggiungi questa dipendenza al tuo file "Cargo.toml"
// [dependencies]
// clap = "2"

use std::io;
use terminal::{Action, Clear};


fn main() {
    pulisci_schermo();
    loop {
        //chiamo e stampa il menu
        menu();
        let mut scelta = String::new();

        io::stdin()
            .read_line(&mut scelta)
            .expect("Failed to read input");
        pulisci_schermo();

        let trimmed = scelta.trim();
        // todo esegui comando
        let scelta = match trimmed.parse::<i32>() {
            Ok(i) if (i == 1 || i == 2) => ElencoScelte::from(i),
            Ok(i) if i == 0 => ElencoScelte::from(i),
            Ok(_) => ElencoScelte::from(-1),
            Err(_) => ElencoScelte::from(-1),
        };
        
        //println!("{scelta}");
        if let Some(_)=esegui_scelta(&scelta) {
            return;
        }
    }
}

fn pulisci_schermo() {
    let terminal = terminal::stdout();

    // perform an single action.
    terminal.act(Action::ClearTerminal(Clear::All)).unwrap();
    terminal.batch(Action::MoveCursorTo(0, 0)).unwrap();
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
    fn from(par_str: i32) -> Self {
        match par_str {
            1 => ElencoScelte::EliminaFile,
            2 => ElencoScelte::EliminaCestino,
            0 => ElencoScelte::Esci,
            _ => ElencoScelte::NessunaOperazione,
        }
    }
}
//creao la struct per uscire
struct Uscita;
// aggiunge il parametro &= per riferimento
fn esegui_scelta(par_scelta: &ElencoScelte) -> Option<Uscita>{
    
    //match scelta per 3 opzioni + fuori indice
    match par_scelta {
        ElencoScelte::EliminaFile => {
            println!("Elimino il file");
            None
        }
        ElencoScelte::EliminaCestino => {
            println!("Elimino il cestino");
            None
        }

        ElencoScelte::Esci => {
            println!("Uscita dalla routine");
            //exit(0);
            Some(Uscita)
            
        }

        ElencoScelte::NessunaOperazione => {
            println!("Nessuna operazione è impostata per questa scelta");
            None
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