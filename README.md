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
