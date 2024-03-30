set shell := ["nu", "-c"]
set quiet
alias b := build 
alias r := run

build: 
  cargo b --release

run: build
  cargo r --release --bin main o> res.ppm