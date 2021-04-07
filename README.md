# hb
Heartbeat CLI

## Usage

Here is an example. The result is the same as `seq 1000000`.

```bash
seq 1000000 | hb | hb -d
```

## Help

```txt
hb 0.1.0
Heartbeat

USAGE:
    hb [FLAGS] [OPTIONS]

FLAGS:
    -d, --decode     Decode
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -n, --interval <seconds>    Seconds to send a heartbeat [default: 30]
```
