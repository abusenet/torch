// For custom deserializer.
use serde::{de, Deserialize, Deserializer};
use std::fmt;
use std::fs;
use std::io::Read;
use std::marker::PhantomData;

use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct Node(String, i64);

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Hashes {
    sha1: Option<String>,
    #[serde(alias = "md5")]
    md5sum: Option<String>,
    crc32: Option<String>,
    sha256: Option<String>,
    whirlpool: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct File {
    // A vector of path segments.
    #[serde(alias = "Path")]
    #[serde(deserialize_with = "string_or_seq_string")]
    path: Vec<String>,
    #[serde(alias = "Size")]
    length: i64,
    #[serde(alias = "Hashes")]
    extra_fields: Option<Hashes>,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct Info {
    name: String,
    #[serde(with = "serde_bytes")]
    pieces: Vec<u8>,
    #[serde(rename = "piece length")]
    piece_length: i64,
    #[serde(default)]
    md5sum: Option<String>,
    #[serde(default)]
    length: Option<i64>,
    #[serde(default)]
    files: Vec<File>,
    #[serde(default)]
    private: Option<u8>,
    #[serde(default)]
    path: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "root hash")]
    root_hash: Option<String>,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct Torrent {
    info: Info,
    #[serde(default)]
    announce: Option<String>,
    #[serde(default)]
    nodes: Option<Vec<Node>>,
    #[serde(default)]
    encoding: Option<String>,
    #[serde(default)]
    httpseeds: Option<Vec<String>>,
    #[serde(default)]
    #[serde(rename = "announce-list")]
    announce_list: Option<Vec<Vec<String>>>,
    #[serde(default)]
    #[serde(rename = "creation date")]
    creation_date: Option<i64>,
    #[serde(rename = "comment")]
    comment: Option<String>,
    #[serde(default)]
    #[serde(rename = "created by")]
    created_by: Option<String>,
    // The .path is not part of spec, but used by this library.
    #[serde(default)]
    path: Vec<String>,
}

pub fn parse_json(input: String) -> Vec<File> {
    serde_json::from_str(&input).unwrap_or_else(|_| panic!("Unable to parse {}", input))
}

pub fn parse_torrent(path: String) -> Torrent {
    let mut torrent_content = Vec::new();
    let mut torrent_file =
        fs::File::open(&path).unwrap_or_else(|_| panic!("Unable to open {}", path));
    torrent_file
        .read_to_end(&mut torrent_content)
        .unwrap_or_else(|_| panic!("Unable to read {:?}", path));
    let torrent = serde_bencode::from_bytes(&torrent_content)
        .unwrap_or_else(|_| panic!("Unable to parse {}", path));
    Torrent {
        path: vec![path],
        ..torrent
    }
}

pub fn checks(files: Vec<File>, torrents: Vec<Torrent>) {
    // Converts the list into a hash map by path for faster lookup.
    let mut files_map: HashMap<String, File> = HashMap::new();
    for file in files {
        files_map.insert(file.path.join("/"), file);
    }

    for torrent in torrents {
        let info = torrent.info;
        println!("{}", torrent.path.join("/"));

        let mut files = info.files;
        // If the torrent only has 1 file, the `files` field is empty,
        // and the `info.length` has the length of that only file.
        if files.is_empty() {
            files.push(File {
                // Use empty path here so we can prepend in the loop.
                path: vec![],
                length: info.length.unwrap(),
                extra_fields: None,
            });
        }

        for file in files {
            let mut path = file.path;
            // Files inside torrent has relative path to the torrent.info.name,
            // so we prepend torrent.info.name for full path.
            path.splice(0..0, [info.name.to_owned()]);
            let path = path.join("/");
            match files_map.get(&path) {
                Some(File { length, .. }) if length == &file.length => {
                    println!("|__ {} ({}) ✅", path, length)
                }
                Some(File { length, .. }) => println!(
                    "|__ {} ({}) ❌ - Actual size {}.",
                    path, file.length, length
                ),
                None => println!("|__ {} ❌ - Not Found", path),
            }
        }
    }
}

// Custom deserializer to convert String to Vec<String>.
fn string_or_seq_string<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: Deserializer<'de>,
{
    struct StringOrVec(PhantomData<Vec<String>>);

    impl<'de> de::Visitor<'de> for StringOrVec {
        type Value = Vec<String>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("string or list of strings")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            Ok(vec![value.to_owned()])
        }

        fn visit_seq<S>(self, visitor: S) -> Result<Self::Value, S::Error>
        where
            S: de::SeqAccess<'de>,
        {
            Deserialize::deserialize(de::value::SeqAccessDeserializer::new(visitor))
        }
    }

    deserializer.deserialize_any(StringOrVec(PhantomData))
}
