# wireguard-vanity-address

Generate Wireguard keypairs with a given prefix string.

[![Build Status][build-status-image]][build-status-url]
[![Deps][deps-status-image]][deps-status-url]
[![Is-It-Maintained-Resolution-Time][iim-resolution-image]][iim-resolution-url]
[![Is-It-Maintained-Open-Issues][iim-open-image]][iim-open-url]
[![Crates.io][crates-io-image]][crates-io-url]
[![Docs.rs][docs-image]][docs-url]
[![License][license-image]][license-url]

[build-status-image]: https://travis-ci.org/warner/wireguard-vanity-address.svg?branch=master
[build-status-url]: https://travis-ci.org/warner/wireguard-vanity-address
[deps-status-image]: https://deps.rs/repo/github/warner/wireguard-vanity-address/status.svg
[deps-status-url]: https://deps.rs/repo/github/warner/wireguard-vanity-address
[crates-io-image]: https://img.shields.io/crates/v/wireguard-vanity-address.svg
[crates-io-url]: https://crates.io/crates/wireguard-vanity-address
[docs-image]: https://docs.rs/wireguard-vanity-address/badge.svg
[docs-url]: https://docs.rs/wireguard-vanity-address
[license-image]: https://img.shields.io/crates/l/wireguard-vanity-address.svg
[license-url]: LICENSE
[iim-resolution-image]: http://isitmaintained.com/badge/resolution/warner/wireguard-vanity-address.svg
[iim-resolution-url]: http://isitmaintained.com/project/warner/wireguard-vanity-address
[iim-open-image]: http://isitmaintained.com/badge/open/warner/wireguard-vanity-address.svg
[iim-open-url]: http://isitmaintained.com/project/warner/wireguard-vanity-address

**This is a fork of https://github.com/warner/wireguard-vanity-address.**

The main differences are as follows:
- Case-sensitive searching;
- Searching multiple prefixes at once;
- Printing the informational header to `stderr`, allowing the program output to be comfortably redirected to a file;
- Customisable iteration count

The [Wireguard VPN](https://www.wireguard.com/) uses Curve25519 keypairs, and
displays the Base64-encoded public key in status displays. I found it hard to
remember which key goes with which target, and the config file doesn't really
support comments or attaching human-memorable names to those keys.

So this tool lets you generate a few million keypairs and print out just the
ones with a given string as a prefix.

## Usage

```
$ cargo install wireguard-vanity-address
$ wireguard-vanity-address DAVE KATE

searching - for the longest prefix, one of every 16777216 keys should match
one core runs at 2.03e6 keys/s, CPU cores available: 16
est yield: 517.7 ms per key, 1.93 keys/s (for the longest prefix)
hit Ctrl-C to stop
private GFMjZuaB4Eneqnf4Pf8SID5YZveUVQPg37BiYqGpfW8=  public KATEN7mzdqR4WT6NjxYhzergAZjNwVKSvvItkIjLeis=
private qGO+MY4WfuJNnK6SQmodbkACxfa9IG4MB0m7BIQ0hEA=  public DAVEIeuq8h8qNy39SBrkgCTPmOMZ1+HSLtDfhlhVq0A=
private yBqHMXe/JbCsX9cF0hxp5Fj2U3hswHJEMblJLhtNlHw=  public KATE5s0e6kHk8LJKbOL8i9B6HWlpXnYwZ7UEQAzMXCg=
private oN7Kg1rvgzxBKhFP0Loh9tArxLaUtLDl6jeW2eH3a3o=  public DAVEgG29ryNiWesHqS22/PFiO1+eFXMgY1In1rkJgTU=
private AKT4BhIoVYjjPL07pfHiON6BJGRMj2VGM0yFGYTjS2M=  public KATESHF9AnjN1lToKrd6HFLDYWt0kUjH2UTKf5G53QE=
private eClZ1mHBOz3yHgrb1gsJBOEOSEGgMl8utIqXFRS700Y=  public KATEWKDFh8/FahbN971PWwK5zVwIY9ArY9puyQJZ1X0=
private kKUujUdp0lL6iwFtzNZNewL5S8vzrHQC7kOMGfve8EM=  public KATEa/dM8KoHgFnZ7t8C3IMZwgznP0FTI8vYThNE+g8=
private 2Ctrd6tLzJnHpW7b+BYDz5d5r8oYsaqMl37j7/zVxFc=  public DAVEjRuojsTDAGMiSkuC2mZAA8gOh9X295NU9ZDeYQw=
private kAD6fc7SS6G3OWAK2FugQs/u/Dofdc6FeDCZJwGC8m0=  public DAVEnskjb+qZmFY4qDSFvJoJRRtdhNvyIMfQouFs1hY=
private aNnkTq658aOneX1xbvBY0XMKSZNA52zOAGCD1lH/F1o=  public KATEwjfiGQJSXfejTsJY9jfnaD0+tsoxN86WLtl4cD0=
private iOpDak87U5YUpkT73aHYGFXzOLgtiMgrgXw+bwvnE2I=  public KATEmEB09faUkjyYPbCxDpLWT0xoxU7CtBjpqHJk9mU=
private uHzsiRTT6oVmvwFSmbX5k7XNTPtKXhSlmQ+yurEY2mw=  public DAVEBtnqQCIMAwzVxEXx64WBAvWSJ02OkljxaBkFkTs=
private yEwo8zz62a0IaSQvDvZ4/pqiIGdmBpy28fBYJKcbsU8=  public KATEmY79sjAUTdqZez8R0JnQtdH5IcEtmECfuo25LSc=
private mMgRuu9hKqygkXAxwhcQu2uCUIj1UKL8UGjM939Tw1E=  public DAVEz2gGSWGD6Ynrq2+qK7qJCJRBm6ktpPLlaCS7FWo=
private eFIJkn4/qYhqrDwxFQ6FJ+nI0IBMLLuRIU0dU4Ih3m4=  public DAVE1IqwP2TqLu6RN3uVHs964WaHZk52cgCb9qXT7RE=
private 6NE7CKGGqpjPkbcIBfzj9FPa8q1yPY9zaw3cQi8183s=  public DAVEZfl7Hxj9Gwq3ky/Fl72mLFLyg1xl58UgFY2AQ2U=
...
```

The tool will run for a long time, printing out more and more candidates as
it goes, so just interrupt the process with Control-C or SIGINT when you've
seen something that you like. There is no saved state or config file, and the
generation process is entirely memoryless, so you don't lose any progress by
interrupting it.

Once you've found a key that you like, copy the private half (on the left)
into your `wg0.conf` config file as the `[Interface] PrivateKey=` field, and
use the public half (on the right) on the other side of that VPN connection
in a `[Peer] PublicKey=` entry.

## Performance

On my 2017 laptop (quad-core 2.8GHz), this tool checks about 60 thousand keys
per second per core. It uses [rayon](https://crates.io/crates/rayon) to
parallelize across all available cores, achieving 240k keys per second.

Only a tiny fraction of those trial keys will match the search string. Each
character of the target string reduces this fraction by a factor of about 32
(case-folded base64 encoding). By allowing the match to start anywhere in the
first ten letters, we increase the hit rate by about 10x.

A four-character string like `dave` means only one out of every (roughly)
150k keys will match, while a five-character string like `carol` reduces that
to one out of 5.6 million. Longer strings will yield fewer candidate keypairs
for a given amount of runtime.

You can run this on multiple machines, but of course you then risk revealing
your private VPN key to any of those machines. There is no support for
managing clusters or anything like that: just install the tool on each worker
machine and run it with the same argument.

Since Wireguard VPN keys are not really public identifiers (you wouldn't
publish them on a web page as you might with Tor's "Onion Addresses", or
Bitcoin addresses), my advice is to stick with a four or five character
search string, and don't try too hard to find a perfect pubkey. You only need
something distinctive enough distinguish between a handful of VPN targets in
a status display.

## License

This is distributed under the MIT license, see [LICENSE](LICENSE.md) for
details.
