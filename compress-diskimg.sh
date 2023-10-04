#!/usr/bin/env bash
# compresses the popcorn disk image with zstd
# using compression level 19

# check if `zstd` is present
if ! command -v zstd; then
	echo "error: command \"zstd\" is not present" 1>&2;
	exit 1;
fi

# check if `cargo` is present
if ! command -v cargo; then
	echo "error: command \"cargo\" is not present" 1>&2;
	exit 2;
fi

# check if `cargo bootimage` is present
if ! command -v bootimage; then
	echo "error: cargo subcommand \"bootimage\" is not present" 1>&2;
	exit 3;
fi

# if all checks have passed, start building the kernel image
# display the logo header first
echo "compress-diskimg.sh - Compresses a bootable Popcorn disk image";
echo "Copyright (c) 2023 The Popcorn Project. See LICENSE for more information";
echo "";

echo "building disk image...";
echo "cargo -v bootimage -v";
# build the bootimage with verbose output
cargo -v bootimage -v;

if [ $? != 0 ]; then
	echo "error: \"cargo bootimage\" has failed." 2>&1;
	exit $(($? + 10));
fi

echo "";
echo "compressing disk image...";
# compress the disk image, with verbose output
echo "yes | zstd -vz19o target/x86_64-arch/debug/bootimage-popcorn.bin.zst target/x86_64-arch/debug/bootimage-popcorn.bin";
yes | zstd -vz19o target/x86_64-arch/debug/bootimage-popcorn.bin.zst target/x86_64-arch/debug/bootimage-popcorn.bin;
if [ $? != 0 ]; then
	echo "error: \"zstd -vz19o target/x86_64-arch/debug/bootimage-popcorn.bin.zst target/x86_64-arch/debug/bootimage-popcorn.bin\" failed" 2>&1;
	exit $(($? + 20));
fi

# the disk image has been compressed. exit is safe
echo "Compressed disk image successfully, written to \"target/x86_64-arch/debug/bootimage-popcorn.bin.zst\"";

# i mean those who work on popcorn must be nocturnal people so...
echo "Popcorn project wishes you a nice day!";
exit 0;
