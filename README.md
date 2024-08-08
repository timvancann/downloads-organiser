# Downloads Organiser

This is a simple cli written in Rust to organise your downloads folder. It moves files from the downloads folder to the specified folder based on the file type.

## Installation 

on *nix with [just](https://github.com/casey/just) which puts the binary in the `~/.local/bin` folder

```bash
just release
```

otherwise you can build the binary and move it manually to the desired location

```bash
cargo build --release
```

# Usage

```
downloads-organiser scan [OPTIONS]

Options:
  -i, --input-directory <INPUT_DIRECTORY>
          The directory to scan, defaults to the user's download directory
  -o, --output-directory <OUTPUT_DIRECTORY>
          The directory to move files to, defaults to the user's download directory
  -s, --settings <SETTINGS>
          The settings file to use, defaults to the default build-in settings
  -b, --bin-others
          Whether to bin all other unmatched files into a separate folder

``` 

# Customisation

You can generate the default settings file by running the following command

```bash
downloads-organiser settings
```

This will generate a `settings.json` file in the current directory. You can then modify this file to your liking and pass it to the cli using the `-s` flag.

# Example

```bash
downloads-organiser scan -s settings.json
