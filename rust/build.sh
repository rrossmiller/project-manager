#!/bin/zsh
rm pm

if [[ $1 == 'r' ]]; then
	echo "building release"
	cargo build --release &&
		mv target/release/pm .
elif [[ $1 == 'd' ]]; then
	echo "deploy release"
	cargo build --release &&
		mv target/release/pm .
	sudo mv pm /usr/local/bin
else
	cargo build &&
		mv target/debug/pm .
fi

