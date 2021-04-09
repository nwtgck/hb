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

Input data chunk is encoded as two types, DATA and HEARTBEAT.

* DATA: 0x0
* HEARTBEAT: 0x1

DATA type has `Length`, which is the length in octets of the `Data`.

```txt
+--------------------+--------------+-------------------- - - - ---+
|    DATA type (8)   |  Length (32) |  Data (Length * 8)   ...     |
+--------------------+--------------+-------------------- - - - ---+
```

HEARTBEAT type has a random octet, which is discarded.

```txt
+--------------------+------------+
| HEARTBEAT type (8) | Random (8) |
+--------------------+------------+
```

* `(N)`: N bits

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
