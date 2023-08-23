// Aggiungi questa dipendenza al tuo file "Cargo.toml"
// [dependencies]
// clap = "2"

use std::io;
use menu_principale::MENU_PRINCIPALE;
use terminal::{Action, Clear};
use traits::*;
mod menu_principale;
mod sottomenu;
mod traits;

fn main() {
    pulisci_schermo();
    // Tutti i menu
    let mut menu_corrente: Box<dyn Menu> = Box::new(MENU_PRINCIPALE);
    loop {
        //chiamo e stampa il menu
        menu_corrente.visualizza();
        
        let scelta = leggi_input();
        
        pulisci_schermo();
        
        let Some(azioni) = menu_corrente.scegli(scelta) else { continue };
        
        
        if let Azioni::CambioMenu(menu) = azioni {
            menu_corrente = menu;
        } else {
            return;   //uscita DAL PROGRAMMA
        }
    }
}

fn leggi_input() -> Option<i32> {
    let mut scelta = String::new();
    io::stdin()
        .read_line(&mut scelta)
        .expect("Failed to read input");

    let trimmed = scelta.trim();
    // todo esegui comando
    return match trimmed.parse::<i32>() {
        Ok(i) => Some(i),
        Err(_) => None, // nessuna operazione
    };
}


//@cls_(pulizia schermo)
fn pulisci_schermo() {
    let terminal = terminal::stdout();

    // perform an single action.
    terminal.act(Action::ClearTerminal(Clear::All)).unwrap();
    terminal.batch(Action::MoveCursorTo(0, 0)).unwrap();
}