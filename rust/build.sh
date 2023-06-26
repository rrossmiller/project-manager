#!/bin/zsh
rm pm

if [[ $1 == 'r' ]]; then
	echo "building release"
	cargo build --release &&
		mv target/release/pm .
	exit 0
elif [[ $1 == 'd' ]]; then
	echo "deploy release"
	cargo build --release &&
		mv target/release/pm .
else
	cargo build &&
		mv target/debug/pm .
	exit 0
fi

sudo mv pm /usr/local/bin
