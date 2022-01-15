use std::env;
use std::fs;
use std::io::{self, Read};

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
    let torrents = args
        .into_iter()
        .map(|torrent_path| {
            let mut torrent_content = Vec::new();
            let mut torrent_file = fs::File::open(&torrent_path).expect("Unable to open file");
            torrent_file
                .read_to_end(&mut torrent_content)
                .expect("Unable to read");
            serde_bencode::from_bytes(&torrent_content).expect("Unable to parse torrent")
        })
        .collect();

    torch::checks(
        serde_json::from_str(&input).expect("Unable to parse JSON"),
        torrents,
    );

    Ok(())
}
