# passgen

A simple random password generator.

## Usage
```
passgen 1.0
@Sparkenstein
Generate Random password on command line

USAGE:
    passgen [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --include <include>    include numbers/upppercase/lowercase/symbols in generated string [default: nlus]
    -l, --length <length>      include lowercase characters in generated string [default: 16]
```

## Example

Generate random password with default settings (16 char long string with number, lowercase, uppercase, and symbols):
```
passgen
```

Change length of password to 20 chars:
```
passgen -l 20
```

Use only Lowercase and Uppercase characters
```
passgen -i lu
```

Generate only number, symbols and uppercase with length of 25
```
passgen -l 25 -i nsu
```