# camtypers

[![Build Status](https://travis-ci.org/kjunichi/camtypers.svg?branch=master)](https://travis-ci.org/kjunichi/camtypers)
[![Build status](https://ci.appveyor.com/api/projects/status/y4bw8wc6baj74de8?svg=true)](https://ci.appveyor.com/project/kjunichi/camtypers)

[![camtypers](http://img.youtube.com/vi/5DliB_wdDEU/0.jpg)](http://www.youtube.com/watch?v=5DliB_wdDEU)

# Prerequisites

## for Windows

```bash
cinst opencv
set LIB=%lib%;C:\tools\opencv\build\x64\vc14\lib
set PATH=%path%;C:\tools\opencv\build\x64\vc14\bin
```

## for macOS  

```bash
brew install opencv
```
# usage

```bash
git clone https://github.com/kjunichi/camtypers.git
cd camtypers
cargo build --release
```
