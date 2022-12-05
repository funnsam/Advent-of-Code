md Day%1-%2
cd Day%1-%2
echo fn main() {} > main.rs
echo all: > Makefile
echo 	rustc main.rs -o day%1-%2.exe -C opt-level=3 -C lto=on >> Makefile
cd ..