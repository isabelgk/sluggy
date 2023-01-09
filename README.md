# sluggy

Create a slugified alphanumeric name with an optional random string appended or prepended to it.

```
$ sluggy "Hello There"
oz130toh-hello-there
```

## Options

```
Usage: sluggy [OPTIONS] [NAME]

Arguments:
  [NAME]  Base name

Options:
  -p, --position <POSITION>  Prepend or append the base name [default: append] [possible values: prepend, append]
  -l <LENGTH>                Sets the length of the random string [default: 8]
  -c, --case <CASE>          Sets the case of the result [default: lowercase] [possible values: lowercase, uppercase, mixed]
  -n, --numbers <NUMBERS>    Where to allow numbers [default: not-first] [possible values: none, anywhere, not-first]
  -h, --help                 Print help information (use `--help` for more detail)
  -V, --version              Print version information
```

