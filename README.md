# rot13

## Usage

* Invoke the binary
* Input your text
* Text will be rot13 encoded
* Cipher text will be printed
* Cipher text will be copied to clipboard with wl-copy (if possible)

```
$ rot13
input: [your text here]
rot13: [lbhe grkg urer]
Copied via wl-copy with PID: 136266
```

## Install

Dependencies:
* wl-copy (optional, removes need for manual copying)

```
$ git clone https://github.com/chellipse/rot13
$ cd rot13
$ cargo install --path .
```

