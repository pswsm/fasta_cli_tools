# Fasta CLI Toolkit
A small CLI toolkit for interacting with fasta file developed in Rust.

As of 21-04-2022 there are no checks on wether the file is actually a fasta file.

## Syntax
### Printing files
```sh
$ fasta_cli_toolkit print /path/to/file
```
Reads the file and outputs its contents

### Cutting sequences
```sh
$ fasta_cli_toolkit cut start end /path/to/input/file /path/to/output/file
```
Reads the file into a Fasta struct, then cuts the sequence, and writes to ouput file

### Format
```sh
$ fasta_cli_toolkit format /path/to/input/file [/path/to/output/file]
```
Reads teh file into a Fasta struct, formats using TextWrap, and outputs the formatted text. If output file is provided, writes to it.

### Generate
```sh
$ fasta_cli_toolkit generate N /path/to/output/file
```
Generates a random DNA sequence long N lines and writes it to output file.
