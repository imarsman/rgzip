# rgzip
GZip in rust

This is my first Rust program and as such is not gong to be sophisticated. It is
intended to do the same things as my gogzip program. My goal is to compare the
pros and cons of Rust versus Go for this task. Rust seems like a pretty good fit
for commandline utilities but I get the feeling that Go is a lot more
confortable to use for things like web services and encryption with TLS
certificates.

I've spent about 3 hours on this, including the time to install Rust, add in the
VS Code support, figure out what the heck cargo is, find out where things go
when they are compiled, etc. I see that Rust has the ability to do testing. That
is good. We'll see.

I'm finding error handling to be a bit of a learning experience. It looks to be
quite capable but seems to be using a different underlying philosophy than I'm
used to.

## Arguments

* `rgzip -h` - print usage
* `rgzip <file>` - gzip a file and remove the original
* `rgzip -f <file>` - gzip a file and overwrite if it is there already
* `rgzip -k <file>` - gzip a file and do not remove the original
* `rgzip -c <file>` - gunzip a file to stdout


## Usage

I will be adding testing.

To build with debugging

`cargo build`

output goes in `./target/debug/rgzip`

To build for release

`cargo build --release`

output goes in `./target/release/rgzip`


## Simple test to compare with gogzip

Golang gogzip is slightly slower than the Rust rgzip release build. Both are
slower than the built-in gzip.

The Go version has more lines, but it also has more going on. In general,
though, the Rust version is a lot more concise. If the Rust version had the
extra logic added to it that exists in the Go version it would still likely be
half as many lines of code. This speaks I think partly to the languages
themselves and also I am sure, my programming.

```
time for i in {1..1000}; do rgzip -k -f -i sample/*.txt; done

real	0m6.662s
user	0m3.735s
sys	0m1.848s
```

```
$: time for i in {1..1000}; do gogzip -f -k sample/*.txt; done

real	0m8.141s
user	0m4.123s
sys	0m3.811s
```

Here is the native one

```
$: time for i in {1..1000}; do gzip -f -k sample/*.txt; done

real	0m4.451s
user	0m1.428s
sys	0m2.852s
```
