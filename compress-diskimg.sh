#!/usr/bin/env bash
# compresses the popcorn disk image with zstd
# using compression level 19

# colors
declare -A COLORS;

COLORS[error]='\e[1;31m';
COLORS[info]='\e[1m';
COLORS[verbose]='\e[1;36m';
COLORS[reset]='\e[0m';

# check if `zstd` is present
if ! command -v zstd; then
	echo -e "[${COLORS[error]}ERROR${COLORS[reset]}] command \"zstd\" is not present" 1>&2;
	exit 1;
fi

# check if `cargo` is present
if ! command -v cargo; then
	echo -e "[${COLORS[error]}ERROR${COLORS[reset]}] command \"cargo\" is not present" 1>&2;
	exit 2;
fi

# check if `cargo bootimage` is present
if ! command -v bootimage; then
	echo -e "[${COLORS[error]}ERROR${COLORS[reset]}] cargo subcommand \"bootimage\" is not present" 1>&2;
	exit 3;
fi

# if all checks have passed, start building the kernel image
# display the logo header first
echo -e "${COLORS[info]}[INFO]${COLORS[reset]} building disk image...";
echo -e "${COLORS[info]}[${COLORS[verbose]}EXEC${COLORS[reset]}${COLORS[info]}]${COLORS[reset]} cargo -v bootimage -v";
# build the bootimage with verbose output
exec cargo -v bootimage -v;

if [ $? != 0 ]; then
	echo -e "[${COLORS[error]}ERROR${COLORS[reset]}] \"cargo bootimage\" has failed." 2>&1;
	exit $(($? + 10));
fi

echo "";
echo -e "${COLORS[info]}[INFO]${COLORS[reset]} compressing disk image...";
# compress the disk image, with verbose output
echo -e "${COLORS[info]}[${COLORS[verbose]}EXEC${COLORS[reset]}${COLORS[info]}]${COLORS[reset]} yes | zstd -vz19o target/x86_64-arch/debug/bootimage-popcorn.bin.zst target/x86_64-arch/debug/bootimage-popcorn.bin";
exec yes | zstd -vz19o target/x86_64-arch/debug/bootimage-popcorn.bin.zst target/x86_64-arch/debug/bootimage-popcorn.bin;
if [ $? != 0 ]; then
	echo -e "[${COLORS[error]}ERROR${COLORS[reset]}] \"zstd -vz19o target/x86_64-arch/debug/bootimage-popcorn.bin.zst target/x86_64-arch/debug/bootimage-popcorn.bin\" failed" 2>&1;
	exit $(($? + 20));
fi

# the disk image has been compressed. exit is safe
echo -e "${COLORS[info]}[INFO]${COLORS[reset]} Compressed disk image successfully, written to \"target/x86_64-arch/debug/bootimage-popcorn.bin.zst\"";

# i mean those who work on popcorn must be nocturnal people so...
echo -e "${COLORS[info]}[INFO]${COLORS[reset]} popcorn project wishes you a nice day!";
exit 0;
