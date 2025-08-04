# rxxd

**Rxxd** is a basic xxd-like hexdump tool written as a learning exercise.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

## Overview

This utility reads data from a specified file or from standard input and outputs its hexadecimal representation.
It supports customizable columns, group sizes, byte ordering, and optional colorization.

## Features

- **Flexible byte grouping**: set the number of bytes per line and group them as desired.
- **Endian control**: display bytes in Big or Little Endian order.
- **Partial reading**: optionally specify an offset or a limited number of bytes.
- **Colorized output**: improved readability with optional colorized output.
- **File and STDIN support**: read from a file or fall back to STDIN.
- **Custom formatting**: specify columns per line and group size.

## Command Line Options

The utility makes use of the clap crate to provide a useful CLI. Summary of available options:

```sh
-f, --filename <FILENAME>: File to be dumped. If not provided, the program reads from stdin.
-c, --cols <COLS>: Number of octets (bytes) per line (default: 16).
-g, --groupsize <GROUPSIZE>: Number of bytes in each group (default: 2).
-l, --len <LEN>: Number of bytes to read (default: read entire input).
-s, --seek <SEEK>: Start at <seek> bytes from the beginning of the file (default: 0).
--endian: Display bytes in Little Endian order (default is Big Endian).
--color: Enable colorized output.
```

## Future Additions

- Implement xxd's revert feature to enable the conversion of an rxxd hexdump into its original binary.
