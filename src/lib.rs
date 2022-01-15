// For custom deserializer.
use serde::{de, Deserialize, Deserializer};
use std::fmt;
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
    #[serde(alias = "Path")]
    #[serde(deserialize_with = "string_or_seq_string")]
    path: Vec<String>,
    #[serde(alias = "Size")]
    length: u64,
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
}

pub fn checks(files: Vec<File>, torrents: Vec<Torrent>) {
    let mut files_map: HashMap<Vec<String>, File> = HashMap::new();
    for file in files {
        files_map.insert(file.path.clone(), file);
    }

    for torrent in torrents {
        for file in torrent.info.files {
            let path = file.path.clone();
            match files_map.get(&path) {
                Some(File { length, .. }) if length == &file.length => {
                    println!("path and lenth matched {:?}", file)
                }
                Some(file) => println!("length not matched{:?}", file),
                None => println!("{:?} is not found.", path),
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
