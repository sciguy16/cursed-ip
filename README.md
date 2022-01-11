# IP address to UUID converter

Handy command-line program for your most common data conversion needs!

## IP address to UUID
```bash
$ cargo run -- 2001:db8::abcd:123:1
20010db8-0000-0000-0000-abcd01230001
```

## UUID to IP address
```bash
$ cargo run -- 02b58300-b8b9-4aaf-a225-d0c9b5161483
2b5:8300:b8b9:4aaf:a225:d0c9:b516:1483
```

## Legacy IP to UUID
```bash
$ cargo run -- 192.2.0.65
00000000-0000-0000-0000-ffffc0020041
```

## License
This software is available under the terms of the [Mozilla Public License 2.0](https://choosealicense.com/licenses/mpl-2.0/).
