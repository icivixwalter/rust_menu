/*
https://runebook.dev/it/docs/rust/book/ch12-01-accepting-command-line-arguments
La funzione args e Unicode non valido
 portiamo il modulo std::env nello scope con un'istruzione use in modo da poter usare la sua funzione args .
 Notare che la funzione std::env::args è annidata in due livelli di moduli
  nei casi in cui la funzione desiderata è nidificata in più di un modulo,
  è convenzionale portare il modulo padre nell'ambito piuttosto che la funzione. In tal modo, possiamo facilmente utilizzare altre funzioni da std::env .
   UNICODE =  std::env::args andrà in panico se un argomento contiene Unicode
   UNICODE NON VALIDO = usa invece std::env::args_os . Quella funzione restituisce un iteratore che produce valori OsString invece di valori String
    Abbiamo scelto di utilizzare std::env::args qui per semplicità, perché i valori OsString
    differiscono per piattaforma e sono più complessi con cui lavorare rispetto ai valori String .

    TRASFORMARE L'ITERATORE IN UN VETTORE
    env::args e usiamo immediatamente collect per trasformare
    l'iteratore in un vettore contenente tutti i valori prodotti dall'iteratore.
    Possiamo usare la funzione collect per creare molti tipi di raccolte,
    quindi annotiamo esplicitamente il tipo di args per specificare che vogliamo un vettore di stringhe
    NECESSARIO ANNOTARE IL TIPO per facilitare rust a definire la raccolta del vettore.

  */

// //use std::process::Command;
// use std::env;
// //use file_time::FileTime;
// use std::fs;

// use clap::Parser;

// /*COSTANTI PATH ARRIVO E PARTENZA PER ORA SI TROVANO QUI C:\\CASA\PROGRAMMI\\RUST_COMPRIMI\\resources\\
//   ma con il percorso relativo dovrebbe funzionare sempre ovunque dove sposti exe e i txt :
//     .\\resources\\path_Partenza.txt"    @path.relativa   @percorso.relativo   @risorse.txt
//     @txt.risorse,   @partenza_(La @path di partenza nel file @esterno)
// */

                              
// const FILE_PARTENZA: &str = ".resouces/EXPLORER/path_Partenza.txt";

// /// Simple app for backup files and folders recursively from a file with a list of paths
// #[derive(Parser, Debug)]
// #[clap(author, version, about, long_about = None)]
// struct Argomenti {
//     // short: il parametro corto (-i), long: il parametro con nome completo --input-file-with-paths
//     #[clap(short = 'i', long)]
//     input_path: String,
// }

// pub fn apri_explorer() {
//     // leggo gli argomenti con l'iteratore arg ---> collect che vengono trasformati in un vettore
//     // let args: Vec<String> = env::args().collect();

//     //******************************* aggiunto per i parametri */
//     //se da linea di comando inserisci -i e -o prende i valori
//     //dai parametri altrimenti prende quelli dai file .txt
//     //select case con 2 bracci
//     let args = match Argomenti::try_parse() {
//         Ok(arg) => arg,
//         Err(_) => match fs::read_to_string(FILE_PARTENZA) {
//             Ok(file) => Argomenti { input_path: file },
//             Err(_) => panic!("Il file presente indicato nei parametri .txt non si trova!!"),
//         },
//     }; // salva gli argomenti CLI in una nuova istanza della struct

//     //***++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++ */
//     let path = env::current_dir().unwrap();
//     println!(
//         "Il path corrente {}",
//         path.display()
//     );

//     // open a file
//     match opener::open(&args.input_path) {
//         Ok(()) => println!("il file {} è stato trovato ed aperto", &args.input_path),
//         Err(_) => panic!("Il file {} non si trova!!", &args.input_path),
//     };
// }
