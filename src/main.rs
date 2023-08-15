// Aggiungi questa dipendenza al tuo file "Cargo.toml"
// [dependencies]
// clap = "2"

use crate::menu_principale::MenuPrincipale;
use std::io;
use sottomenu::Sottomenu;
use terminal::{Action, Clear};
use traits::*;
mod menu_principale;
mod sottomenu;
mod traits;

fn main() {
    pulisci_schermo();
    // Tutti i menu
    let menus: Vec<Box<dyn traits::Menu>> = vec![
        Box::new(MenuPrincipale), 
        Box::new(Sottomenu)
        // TODO: inserire i nuovi menu in ordine qui dentro!!!
    ];
    let mut menu_corrente = menus.first().unwrap();
    loop {
        //chiamo e stampa il menu
        menu_corrente.visualizza();
        
        let scelta = leggi_input();
        
        pulisci_schermo();
        
        let Some(azioni) = menu_corrente.scegli(scelta) else { continue };
        
        menu_corrente = match azioni {
            Azioni::Uscita => std::process::exit(0),
            Azioni::CambioMenu(menu) => menus.get(menu as usize).unwrap(),
        };
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

fn pulisci_schermo() {
    let terminal = terminal::stdout();

    // perform an single action.
    terminal.act(Action::ClearTerminal(Clear::All)).unwrap();
    terminal.batch(Action::MoveCursorTo(0, 0)).unwrap();
}