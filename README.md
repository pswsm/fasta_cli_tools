# Fasta CLI Toolkit
A small CLI toolkit for interacting with fasta files developed in Rust.

As of 2022-08-29 there are no checks on wether the file is actually a fasta file, since it's a plain text file.

## TODO
I won't set any specific dates because I'm doing this on my free time.

- DNA to Amino acid translation
- ~DNA transcrption and translation~
- Amino Acid generation
---------------------------
Down here harder things, to my assesment
- Alignement between two sequences
    I'd rather do Smith-Waterman first, feels easier
    Then someday I'll make Needleman-Wunsch
- Blastn ----> Through NCBI database?


## Syntax
### Printing files
Reads the file and outputs its contents
```sh
$ fasta_cli_toolkit print /path/to/file
```

### Cutting sequences
Reads the file into a Fasta struct, then cuts the sequence, and writes to ouput file
```sh
$ fasta_cli_toolkit cut start end /path/to/input/file /path/to/output/file
```

### Format
Reads the file into a Fasta struct, formats using TextWrap, and outputs the formatted text. If output file is provided, writes it.
```sh
$ fasta_cli_toolkit format /path/to/input/file [/path/to/output/file]
```

### Generate
Generates a random DNA sequence long N lines and writes it to output file.
```sh
$ fasta_cli_toolkit generate N /path/to/output/file [-r|--rna]
```
Use the `-r` or `--rna` options to generate a RNA sequence

### Analyze
Prints a summary of the sequence:
 - Number of bases
 - AT Count & Percentage
 - GC Count & Percentage
```sh
$ fasta_cli_toolkit analyze /path/to/file
```

### Get
Get subcommands:
 - complementary:
    Gets the complementary strand of the given sequence
    ```sh
    $ fasta_cli_toolkit get complementary /path/to/file [/path/to/output/file]
    ```
 - reverse
    Gets the reverse strand of the given sequence
    ```sh
    $ fasta_cli_toolkit get reverse /path/to/file [/path/to/output/file]
    ```
 - revcomp
    Gets the reverse-complementary strand of the given sequence
    ```sh
    $ fasta_cli_toolkit get revcomp /path/to/file [/path/to/output/file]
    ```
