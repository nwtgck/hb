use std::io;
use std::io::Write;
use std::time::Duration;
use structopt::StructOpt;

// Same value as used in std::io::copy()
pub const DEFAULT_BUF_SIZE: usize = 8 * 1024;

const DATA_FLAG: u8 = 0;
const HEARTBEAT_FLAG: u8 = 1;

fn encode<R: ?Sized>(reader: &mut R) -> io::Result<()>
where
    R: io::Read,
{
    let stdout = &mut io::stdout();
    // TODO: use MaybeUninit
    let mut buf: [u8; DEFAULT_BUF_SIZE] = [0; DEFAULT_BUF_SIZE];
    loop {
        let len = match reader.read(&mut buf) {
            Ok(0) => return Ok(()),
            Ok(len) => len,
            Err(ref e) if e.kind() == std::io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };
        {
            let mut stdout_lock: io::StdoutLock = stdout.lock();
            stdout_lock.write_all(&[DATA_FLAG])?;
            stdout_lock.write_all(&(len as u32).to_be_bytes())?;
            stdout_lock.write_all(&buf[..len])?;
            stdout_lock.flush()?;
        }
    }
}

fn decode<R: ?Sized, W: ?Sized>(reader: &mut R, writer: &mut W) -> io::Result<()>
where
    R: io::Read,
    W: io::Write,
{
    // TODO: use MaybeUninit
    let mut buf: [u8; DEFAULT_BUF_SIZE] = [0; DEFAULT_BUF_SIZE];
    let mut one_buf: [u8; 1] = [0];
    let mut len_buf: [u8; 4] = [0; 4];
    let mut rest: usize = 0;
    loop {
        if rest == 0 {
            let flag = match reader.read(&mut one_buf) {
                Ok(0) => return Ok(()),
                Ok(_) => one_buf[0],
                Err(e) => return Err(e),
            };
            match flag {
                DATA_FLAG => {
                    reader.read_exact(&mut len_buf)?;
                    rest = u32::from_be_bytes(len_buf) as usize;
                }
                HEARTBEAT_FLAG => {
                    // discard one byte
                    reader.read_exact(&mut one_buf)?;
                    continue;
                }
                // TODO: return Err
                _ => panic!("unexpected flag: {}", flag),
            }
        }
        let read_max = buf.len().min(rest);
        let read_len = reader.read(&mut buf[..read_max])?;
        if read_len == 0 {
            return Ok(());
        }
        rest -= read_len;
        writer.write_all(&buf[..read_len])?;
        writer.flush()?;
    }
}

/// Heartbeat
#[derive(StructOpt, Debug)]
#[structopt(name = "hb")]
#[structopt(rename_all = "kebab-case")]
struct Opt {
    #[structopt(short = "d", long = "decode")]
    /// Decode
    decodes: bool,
    // Same short flag name as watch command
    #[structopt(short = "n", long = "interval", name = "seconds", default_value = "30")]
    /// Seconds to send a heartbeat
    interval_seconds: u64,
}

fn main() {
    // Parse options
    let opt = Opt::from_args();
    let Opt {
        decodes,
        interval_seconds,
        ..
    } = opt;
    if decodes {
        decode(&mut io::stdin(), &mut io::stdout()).unwrap();
    } else {
        std::thread::spawn(move || loop {
            std::thread::sleep(Duration::from_secs(interval_seconds));
            // Send heartbeat
            let stdout = &mut io::stdout();
            let mut stdout_lock = stdout.lock();
            stdout_lock
                .write(&[HEARTBEAT_FLAG, rand::random::<u8>()])
                .unwrap();
            stdout_lock.flush().unwrap();
        });
        encode(&mut io::stdin()).unwrap();
    }
}
