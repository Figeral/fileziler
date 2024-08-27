# Fileziler

Fileziler is a command-line utility written in Rust that allows you to compress and decompress files using the Gzip algorithm. It supports files of any type and size, making it a versatile tool for file management.

## Features
+ Compress files: Reduce the size of your files using Gzip compression.
+ Decompress files: Restore compressed files to their original state.
+ Efficient: Handles large files with ease.
   
## Usage
Here’s how you can use the fileziler utility from the command line:
```sh
./fileziler --opt <OPT> --path <PATH>
```

## Options

+ `-o, --opt <OPT>`
  Specify the operation to perform. Possible values:
  + `compress`: Compress the file at the specified path.
  + `decompress`: Decompress the file at the specified path. 
+ `-p, --path <PATH>`
    Specify the path to the file you want to compress or decompress.
+ `-h, --help`
    Display help information about the fileziler command.

## Examples

### compress 
To compress a file named `example.txt`:
```sh
./fileziler --opt compress --path ./path/example.txt
```
This will create a example.gzip file in the same directory.

## decompress 
To decompress a file named `example.gzip`:
```sh
fileziler --opt decompress --path ./example.gzip
```
This will restore the original example.txt file.