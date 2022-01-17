# Torch 🔦

It is dark out there 👀. Bring 🔦 with you to lighten the paths.

This is a tool to check the files inside one or many torrents against a listing
for existence and correct size.

## Usage

```sh
# Checks the content against a local JSON
$ cat list_to_check.json | torch *.torrent
# Checks the content against files on remote source through `rclone`
$ rclone lsjson source: --recursive | torch *.torrent
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

`torch` displays all files checked, whether they are ok or not. To see only
files with errors, pipe through `grep`, i.e.:

```shell
torch *.torrent | grep '❌'
```

For torrent files created with BitComet, they can have padding with folders
inside `.pad`, and may be reported as missing. In that case, they can be filtered
out using `grep`, i.e.:

```shell
torch *.torrent | grep -v '.pad/'
```

If it has not been clear enough, `torch` is made as a small Unix tool, so it only
does one thing, and should be combined with other tools to make it more useful.

## Examples

```
Downloads
|_ Big Buck Bunny
  |_ Big Buck Bunny.en.srt
  |_ Big Buck Bunny.mp4
```

```shell
$ cd ~/Downloads
$ rclone lsjon . --recursive | torch *.torrent
big-buck-bunny.torrent
|__ Big Buck Bunny/Big Buck Bunny.en.srt (140) ✅
|__ Big Buck Bunny/Big Buck Bunny.mp4 (276134947) ❌ - Actual size 276134
|__ Big Buck Bunny/poster.jpg (310380) ❌ - Not Found
```
