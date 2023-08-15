// unit struct che serve solo a definire se uscire o no dal programma.
pub enum Azioni {
    Uscita,
    CambioMenu(i32),
}

pub trait Scelte {}

// interfaccia generale per i menu
pub trait Menu {
    // una enum con le opzioni del menu, che implementi almeno IntoEnumIterator e Debug
    fn get_iter(&self) -> Box<dyn Iterator<Item = String>>;
    fn get_name(&self) -> String;
    // metodo di default che stampa tutte le opzioni del menu
    fn visualizza(&self) {
        println!("***** {} *****", self.get_name());
        for (i, s) in self.get_iter().enumerate() {
            if !s.eq("NessunaOperazione") {
                println!("[{}] {:?}", i, s);
            }
        }
    }
    // metodo astratto che deve implementare le operazioni da effettuare a ogni scelta del menu
    fn scegli(&self, scelta: Option<i32>) -> Option<Azioni>;
}
