use crate::sottomenu::SOTTOMENU;
use crate::traits::*;
// use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use strum_macros::EnumString;
use strum_macros::Display;

// deve essere un nuovo intero, diverso dai precedenti.
pub const MENU_PRINCIPALE: i32 = 0;

// i trait da implementare:
// Default: rende disponibile il metodo ::default(), che permette di creare una nuova ScelteMenuPrincipale::NessunaOperazione
// Debug: rende possibile stampare il valore dell'enum con :? in un println!("{:?}")
// EnumString: permette di convertire le varianti dell'enum in stringhe. Utile per stampare l'operazione scelta.
// EnumIter: permette di ottenere un iteratore sui valori dell'enum, tranne quelli con #[strum(disabled)]. Utile per stampare il menu
#[derive(Default, Debug, Display, EnumString, EnumIter)]
#[repr(i32)]
pub enum ScelteMenuPrincipale {
    Esci = 0,
    EliminaFile,
    EliminaCestino,
    SottoMenu,
    #[default]
    NessunaOperazione,
}
impl Scelte for ScelteMenuPrincipale{}

pub struct MenuPrincipale;
impl Menu for MenuPrincipale {
    
    fn get_iter(&self) -> Box<dyn Iterator<Item = String>>{
        Box::new(ScelteMenuPrincipale::iter().map(|e| e.to_string()))
    }
    
    fn get_name(&self) -> String{
        "Menu Principale".to_string()
    }
    
    fn scegli(&self, scelta: Option<i32>) -> Option<Azioni> {
        // trasforma il parametro di input in una variante dell'Enum S
        let scelta = match scelta {
            Some(i) => ScelteMenuPrincipale::iter().nth(i as usize).unwrap_or_default(),
            None => ScelteMenuPrincipale::default(),
        };
        
        println!("{}", scelta);
        // implementa le azioni di ogni voce di menu. Restituisci Some(Uscita) per terminare il programma, None altrimenti.
        match scelta {
            ScelteMenuPrincipale::EliminaFile => elimina_file(),
            ScelteMenuPrincipale::EliminaCestino => elimina_cestino(),
            ScelteMenuPrincipale::SottoMenu => Some(Azioni::CambioMenu(SOTTOMENU)),
            ScelteMenuPrincipale::NessunaOperazione => None,
            ScelteMenuPrincipale::Esci => Some(Azioni::Uscita),
        }
    }
}


fn elimina_file() -> Option<Azioni> {
    // Restituisci None se non vuoi che il programma termini, altrimenti Some(Uscita)
    None
}

fn elimina_cestino() -> Option<Azioni> {
    // Restituisci None se non vuoi che il programma termini, altrimenti Some(Uscita)
    None
}