# Torch 🔦

It is dark out there 👀. Bring 🔦 with you to lighten the paths.

This is a tool to check the files inside one or many torrents against a listing
for existence and correct size.

## Usage

```sh
# Checks the content against a local JSON
$ cat list_to_check.json | torch *.torrent
# Checks the content against files on remote source through `rclone`
$ rclone lsjson source: | torch *.torrent
```

The JSON file must contain an array of `File` with at least the following fields:

```json
{
  "path" : "full/path/goes/here/file.txt",
  "length" : 6
}
```

The following fields are also acceptable, making it compatible with rclone's
`lsjson` output:

```json
{
  "Path" : "full/path/goes/here/file.txt",
  "Size" : 6
}
```
