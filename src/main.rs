use std::env;
use std::io::{self, Read};
use torch::{checks, parse_json, parse_torrent};

fn main() -> io::Result<()> {
    // Reads list of files to check from stdin.
    let mut stdin = io::stdin();
    let mut input = String::new();
    stdin
        .read_to_string(&mut input)
        .expect("Unable to read from stdin");

    // Parses torrent files from arguments.
    let mut args: Vec<String> = env::args().collect();
    // remove first argument which is self
    args.remove(0);
    let torrents = args.into_iter().map(parse_torrent).collect();

    checks(parse_json(input), torrents);

    Ok(())
}
