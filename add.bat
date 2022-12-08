@echo off

cd 20%1
md Day%2-1
cd Day%2-1
echo use std::fs;> main.rs
echo. >> main.rs
echo fn main() {>> main.rs
echo     let input: String = fs::read_to_string("input.txt").unwrap();>> main.rs
echo }>> main.rs
echo all: > Makefile
echo 	rustc main.rs -o day%2-1.exe -C opt-level=3 -C lto=on >> Makefile
echo. > input.txt

cd ..
md Day%2-2
cd Day%2-2
echo use std::fs;> main.rs
echo. >> main.rs
echo fn main() {>> main.rs
echo     let input: String = fs::read_to_string("input.txt").unwrap();>> main.rs
echo }>> main.rs
echo all: > Makefile
echo 	rustc main.rs -o day%2-2.exe -C opt-level=3 -C lto=on >> Makefile
echo. > input.txt
cd ..\..

@echo on