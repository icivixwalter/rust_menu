#!/bin/bash

clear

echo "prima compilo e poi salvo"

cargo build --target x86_64-pc-windows-gnu

cp ./target/x86_64-pc-windows-gnu/debug/menu_clap.exe ~/CASA/CARTELLACONDIVISA/menu_clap.exe

echo "Cartella condivisa ora contiene:"
ls ~/CASA/CARTELLACONDIVISA/menu*

