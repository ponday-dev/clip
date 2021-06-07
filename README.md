# clip

`clip` is a simple command-line utility to read or write clipboard.

## How to use

Most commonly usage is writing to clipboard from CUI.

```bash
# write 'foo'
$ clip 'foo'
```

`clip` also supports an input from pipe.

```bash
$ cat foo.txt | clip
```

If you want to read clipboard, you can use `--paste` or `-p` option.

```bash
$ clip '{ "name": "John, Smith", "age": 20 }'
$ clip -p | jq
```
