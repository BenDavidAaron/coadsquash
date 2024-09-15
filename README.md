# codesquash

A tool for packing source code into single files for LLM consumption.

## What it does

* Traverses git repostories
* Concats all source code into stdout

## What you do

### First Time
1. Clone it

```
git clone git@github.com:BenDavidAaron/coadsquash.git
```

2. Build it

```
cd codesquash
cargo build --release
```

3. install it

```
sudo cp ./target/release/codesquash /usr/local/bin/
```


### Every Time

Package the git repo you're in

```
codesquash > /tmp/output.txt
```

Package another repo on your system
```
codesquash /path/to_other/repo > /tmp/output.txt
```