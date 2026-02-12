# vibe-coded

`vibe-coded` is a simple Rust utility that determines if a repository is
genuine human work or just a simple prompt expansion. It clones a target
repository and runs a set of rules against it to see if it passes the
vibe check.

# Installation 

## Pre-built Binaries 

Grab the latest binary for your system from the
[Releases](https://github.com/exlee/vibe-coded/releases) page.

## From Source 

If you have Rust installed, you can build it yourself:

``` bash
cargo install --path .
```

# Usage 

It takes just one argument - the path to the repository you want to
check:

``` bash
vibe-coded https://github.com/username/repo
```

# The Vibe (Philosophy & Rules) 

It is getting harder to figure out if code is someone's crafted work or
a simple prompt expansion. This tool runs heuristics to measure the
"soul" of the project.

**The rules are arbitrary by design.**

If you are wondering if a specific rule or check belongs here, the
answer is likely "yes." Don't hesitate to make a request or submit a
PR. The goal is to define "vibe coded" together, so if you feel it,
you can add it (or request).
