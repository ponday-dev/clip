use anyhow::Result;
use arboard::Clipboard;
use atty::Stream;
use std::io::{self, Read, Write};
use std::process;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    src: Option<String>,

    #[structopt(short, long)]
    paste: bool,
}

fn read_from_stdin() -> Result<String> {
    let mut buf = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    if let Err(err) = handle.read_to_string(&mut buf) {
        Err(anyhow::Error::from(err))
    } else {
        Ok(buf)
    }
}

fn read_source(src: &Option<String>) -> Result<String> {
    if atty::is(Stream::Stdin) {
        if let Some(value) = src {
            return Ok(value.clone());
        }
    } else {
        return read_from_stdin();
    }

    Err(anyhow::anyhow!("<src> is required"))
}

fn main() -> Result<()> {
    let mut clipboard = Clipboard::new().unwrap();
    let args: Cli = Cli::from_args();

    if args.paste {
        return match clipboard.get_text() {
            Ok(text) => {
                let stdout = io::stdout();
                let mut handle = stdout.lock();

                if let Err(_) = writeln!(handle, "{}", &text) {
                    process::exit(1);
                }
                Ok(())
            }
            Err(err) => Err(anyhow::anyhow!(err)),
        };
    }

    match read_source(&args.src) {
        Ok(text) => match clipboard.set_text(text) {
            Ok(_) => Ok(()),
            Err(err) => Err(anyhow::anyhow!(err)),
        },
        Err(err) => Err(anyhow::anyhow!(err)),
    }
}
