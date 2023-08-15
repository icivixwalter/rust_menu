COMPILAZIONE RUST DA LINUX PER WINDOWS
    Prima di compilare fare l'aggiornamento di rust per windows x86 ma puo essere cambiato il target
        
        rustup target add x86_64-pc-windows-gnu
        
    Prima di compilre, installa
        
        sudo apt install mingw-w64
        
    Poi, aggiungi un file .cargo/config.toml con:
        
        [target.x86_64-pc-windows-gnu]
        linker = "x86_64-w64-mingw32-gcc"
        ar = "x86_64-w64-mingw32-gcc-ar"
        
    Per la compilazione attivare il comando
        cargo build --target x86_64-pc-windows-gnu