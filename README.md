# hb
Heartbeat CLI

## Usage

Here is an example. The result is the same as `seq 1000000`.

```bash
seq 1000000 | hb | hb -d
```

## Install

Download from <https://github.com/nwtgck/hb/releases>

## Spec

Input data chunk is encoded as two types, DATA and HEARTBEAT. The format is as follows.

DATA type has `Length`, which is the length in octets of the `Data`.

```txt
+--------------------+-------------+---------------- - - - ---+
|    DATA type (1)   |  Length (4) |  Data (Length)    ...    |
+--------------------+-------------+---------------- - - - ---+
```

HEARTBEAT type has a random octet, which is discarded.

```txt
+--------------------+------------+
| HEARTBEAT type (1) | Random (1) |
+--------------------+------------+
```

* `(N)`: N octets

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
