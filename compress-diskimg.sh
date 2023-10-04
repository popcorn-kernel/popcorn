#!/usr/bin/env bash
# compresses the popcorn disk image with zstd
# using compression level 19

# check if `zstd` is present
if ! command -v zstd; then
	echo -e "\e[1;31merror:\e[0m command \"zstd\" is not present" 1>&2;
	exit 1;
fi

# check if `cargo` is present
if ! command -v cargo; then
	echo -e "\e[1;31merror:\e[0m command \"cargo\" is not present" 1>&2;
	exit 2;
fi

# check if `cargo bootimage` is present
if ! command -v bootimage; then
	echo -e "\e[1;31merror:\e[0m cargo subcommand \"bootimage\" is not present" 1>&2;
	exit 3;
fi

# if all checks have passed, start building the kernel image
# display the logo header first
echo "compress-diskimg.sh - Compresses a bootable Popcorn disk image";
echo "Copyright (c) 2023 The Popcorn Project. See LICENSE for more information";
echo "";

echo -e "\e[1m[INFO]\e[0m building disk image...";
echo -e "\e[1m[\e[1;36mEXEC\e[0m\e[1m]\e[0m cargo -v bootimage -v";
# build the bootimage with verbose output
cargo -v bootimage -v;

if [ $? != 0 ]; then
	echo -e "\e[1;31merror:\e[0m \"cargo bootimage\" has failed." 2>&1;
	exit $(($? + 10));
fi

echo "";
echo -e "\e[1m[INFO]\e[0m compressing disk image...";
# compress the disk image, with verbose output
echo -e "\e[1m[\e[1;36mEXEC\e[0m\e[1m]\e[0m yes | zstd -vz19o target/x86_64-arch/debug/bootimage-popcorn.bin.zst target/x86_64-arch/debug/bootimage-popcorn.bin";
yes | zstd -vz19o target/x86_64-arch/debug/bootimage-popcorn.bin.zst target/x86_64-arch/debug/bootimage-popcorn.bin;
if [ $? != 0 ]; then
	echo -e "\e[1;31merror:\e[0m \"zstd -vz19o target/x86_64-arch/debug/bootimage-popcorn.bin.zst target/x86_64-arch/debug/bootimage-popcorn.bin\" failed" 2>&1;
	exit $(($? + 20));
fi

# the disk image has been compressed. exit is safe
echo -e "\e[1m[INFO]\e[0m Compressed disk image successfully, written to \"target/x86_64-arch/debug/bootimage-popcorn.bin.zst\"";

# i mean those who work on popcorn must be nocturnal people so...
echo -e "\e[1m[INFO]\e[0m Popcorn project wishes you a nice day!";
exit 0;
