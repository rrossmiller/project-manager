#!/bin/zsh
rm pm 

if [[ $1 == 'r' ]]; then
	echo "building release"
	cargo build --release &&
		mv target/release/pm .
	shift
else
	cargo build &&
		mv target/debug/pm .
fi

if [[ $? -ne 0 ]]; then
	exit 1
fi

clear
./pm $@
rm pm 
