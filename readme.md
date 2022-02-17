<div align = "center">
    <h1>TMSIM Converter</h1>
    <h4>Tool for converting human-readable commands to json</h4>
</div>
<hr>
<br>
<br>

### Requirements

Make shure you have installed
 - `rust`
 - `cargo`
 
If not:
- Download and run rust install for Windows 64 bit - <a href="https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe">link</a>
- Download and run rust install for Windows 32 bit - <a href="https://static.rust-lang.org/rustup/dist/i686-pc-windows-msvc/rustup-init.exe">link</a>
- Linux users can do it by themselves

<div align = "center">
    <h2>Installation</h2>
</div>

```bash
git clone https://github.com/clowzed/tmsim-converter.git
cd tmsim
cargo build --release
sudo mv ./target/release/tmsim-converter.exe /your/bin/dir
# Do not forget to
export PATH="/your/bin/path:$PATH"
```

<div align = "center">
    <h2>Running</h2>
</div>

1) Create a file `conf.tmsim`
```
q0(*) -> q0(*)R
q0(a) -> q0(#)R
q0(b) -> q0(#)R
q0( ) -> q1( )L
q1(#) -> q1(#)L
q1(*) -> q2(*)S


alphabet: (#ab )
tape: (*ab )

```
3) Convert it
```
tmsim-convert conf.tmsim --out conf.json
```
4) Run it!
```bash
tmsim conf.json
```
