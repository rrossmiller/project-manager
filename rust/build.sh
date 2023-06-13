#!/bin/zsh
rm pm

if [[ $1 == 'r' ]]; then
	echo "building release"
	cargo build --release &&
		mv target/release/pm .
	shift
elif [[ $1 == 'd' ]]; then
	echo "deploy release"
	cargo build --release &&
		mv target/release/pm .
else
	cargo build &&
		mv target/debug/pm .
fi

if [[ $? -ne 0 ]]; then
	exit 1
elif [[ $1 == 'd' ]]; then
    exit 0
fi

sudo mv pm /usr/local/bin
