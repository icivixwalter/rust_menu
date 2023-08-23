use std::marker::PhantomData;
    
use crate::sottomenu::*;
use crate::traits::*;
// use std::str::FromStr;
use strum::IntoEnumIterator;
use strum_macros::Display;
use strum_macros::EnumIter;
use strum_macros::EnumString;

// TODO - cambia nome e numero - deve essere un nuovo intero, diverso dai precedenti.  @modello@menu_(menu principale da ricopiare)
pub const MENU_PRINCIPALE: MenuPrincipale<ScelteMenuPrincipale> = MenuPrincipale{scelte: PhantomData};

// i trait da implementare:
// Default: rende disponibile il metodo ::default(), che permette di creare una nuova ScelteMenuPrincipale::NessunaOperazione
// Debug: rende possibile stampare il valore dell'enum con :? in un println!("{:?}")
// EnumString: permette di convertire le varianti dell'enum in stringhe. Utile per stampare l'operazione scelta.
// EnumIter: permette di ottenere un iteratore sui valori dell'enum, tranne quelli con #[strum(disabled)]. Utile per stampare il menu
#[derive(Default, Debug, Display, EnumString, EnumIter, Copy, Clone)]
#[repr(i32)]
pub enum ScelteMenuPrincipale {
    // TODO: cambia nome e varianti
    Esci = 0,               // assicurarsi che la prima sia = 0
    EliminaFile,            // = 1
    EliminaCestino,         // = 2
    SottoMenu,              // = 3
    #[default]
    NessunaOperazione, // lasciare questa qua
}

#[derive(Default)]
pub struct MenuPrincipale<ScelteMenuPrincipale> {
    scelte: PhantomData<ScelteMenuPrincipale>,
}

// impl Default for MenuPrincipale {
//     fn default() -> Self {
//         MenuPrincipale{
//             scelte: ScelteMenuPrincipale::iter().collect(),
//         }
//     }
// }

impl Menu for MenuPrincipale<ScelteMenuPrincipale> {
    // cambia il nome dopo il for

    fn get_iter(&self) -> Box<dyn Iterator<Item = String>> {
        Box::new(ScelteMenuPrincipale::iter().map(|s| s.to_string()))
    }

    
    // cambia la stringa
    fn get_name(&self) -> String {
        "Menu Principale".to_string()
    }

    fn scegli(&self, scelta: Option<i32>) -> Option<Azioni> {
        // trasforma il parametro di input in una variante dell'Enum S
        let scelta = match scelta {
            // cambia le scelte
            Some(i) => ScelteMenuPrincipale::iter().nth(i as usize).unwrap_or_default(),
            None => ScelteMenuPrincipale::default(),
        };

        println!("{}", scelta);
        // implementa le azioni di ogni voce di menu. Restituisci Some(Uscita) per terminare il programma, None altrimenti.
        match scelta {
            // cambia le opzioni e implementale
            ScelteMenuPrincipale::EliminaFile => elimina_file(),
            ScelteMenuPrincipale::EliminaCestino => elimina_cestino(),
            ScelteMenuPrincipale::SottoMenu => Some(Azioni::CambioMenu(Box::new(SOTTOMENU))),
             // questo e' un esempio di cambio menu
            ScelteMenuPrincipale::NessunaOperazione => None, // questa lasciala
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
