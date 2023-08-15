use crate::menu_principale::MENU_PRINCIPALE;
use crate::traits::*;
// use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use strum_macros::EnumString;
use strum_macros::Display;

// deve essere un nuovo intero, diverso dai precedenti.
pub const SOTTOMENU: i32 = 1;

// i trait da implementare:
// Default: rende disponibile il metodo ::default(), che permette di creare una nuova ScelteMenuPrincipale::NessunaOperazione
// Debug: rende possibile stampare il valore dell'enum con :? in un println!("{:?}")
// EnumString: permette di convertire le varianti dell'enum in stringhe. Utile per stampare l'operazione scelta.
// EnumIter: permette di ottenere un iteratore sui valori dell'enum, tranne quelli con #[strum(disabled)]. Utile per stampare il menu
#[derive(Default, Debug, Display, EnumString, EnumIter)]
#[repr(i32)]
pub enum ScelteSottomenu {
    Indietro = 0,
    Opzione1,
    #[default]
    NessunaOperazione,
}
impl Scelte for ScelteSottomenu{}

pub struct Sottomenu;
impl Menu for Sottomenu {
    
    fn get_iter(&self) -> Box<dyn Iterator<Item = String>>{
        Box::new(ScelteSottomenu::iter().map(|e| e.to_string()))
    }
    
    fn get_name(&self) -> String{
        "Sottomenu".to_string()
    }
    
    fn scegli(&self, scelta: Option<i32>) -> Option<Azioni> {
        // trasforma il parametro di input in una variante dell'Enum S
        let scelta = match scelta {
            Some(i) => ScelteSottomenu::iter().nth(i as usize).unwrap_or_default(),
            None => ScelteSottomenu::default(),
        };
        
        println!("{}", scelta);
        // implementa le azioni di ogni voce di menu. Restituisci Some(Uscita) per terminare il programma, None altrimenti.
        match scelta {
            ScelteSottomenu::Indietro => Some(Azioni::CambioMenu(MENU_PRINCIPALE)),
            ScelteSottomenu::Opzione1 => opzione1(),
            ScelteSottomenu::NessunaOperazione => None,
        }
    }
}

fn opzione1() -> Option<Azioni> {
    None
}