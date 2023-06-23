Bully
=====

![GitHub](https://img.shields.io/github/license/nitrogenez/bully)
![GitHub issues](https://img.shields.io/github/issues/nitrogenez/bully)
![GitHub Repo stars](https://img.shields.io/github/stars/nitrogenez/bully)
![GitHub forks](https://img.shields.io/github/forks/nitrogenez/bully)



**Bully** is an open source lock screen brute-force
utility for Android phones. It aims to provide an easy to use CLI,
be fast and flexible.

Bully uses **adb** (Android Debug Bridge) to pass
the generated PIN/passwd combinations to the target device.

> **NOTE**  
> To use Bully you need a physical access to the
> target device in order to pass combinations through USB.

Building
--------
To build Bully you will need `cargo` installed in your system.
To install the Rust development kit, you can either use
the packages from your distribution or the official
`rustup` script: https://rustup.rs.

First of all, you'll need to clone the repository:

```
HTTPS
=====
git clone --depth 1 https://github.com/nitrogenez/bully.git

SSH
===
git clone --depth 1 git@github.com:nitrogenez/bully.git
```

Then, you'll need to cd into the cloned repo:

```
cd ./bully
```

After that, just use `cargo`. It will automatically
download and build all dependencies, and then you will be able
to run Bully by yourself:

```
Optimized build
===============
cargo build --release

Debug build
===========
cargo build --debug
```

And so, you will be able to run Bully:

```
Optimized build
===============
./target/release/bully -h

Debug build
===========
./target/debug/bully -h
```

Usage
-----
Use this command to see help:

```
bully --help
```
