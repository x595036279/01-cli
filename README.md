# RCLI

rcli is a rust cli tool.

## csv
```rust
#[derive(Debug, Parser)]
pub struct CsvOpts {
    // CSV文件
    #[arg(short, long,value_parser = verify_file)]
    pub input: String,
    // 输出的文件
    #[arg(short, long)]
    pub output: Option<String>,
    // 输出文件格式
    #[arg(long,value_parser = parse_format, default_value = "Json")]
    pub format: OutputFormat,

    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,

    #[arg(long, default_value_t = true)]
    pub header: bool,
}
```
> cargo run -- csv -i assets/juventus.csv
> cargo run -- csv -i assets/juventus.csv --format yaml
## b64
```rust
#[derive(Debug, Parser)]
pub enum Base64SubCommand {
    // cargo run -- base64 encode -i Cargo.toml
    #[command(name = "encode", about = "Encode a string to base64")]
    Encode(Base64EncodeOpts),
    // cargo run -- base64 encode --format urlsafe
    #[command(name = "decode", about = "Decode a base64 to String")]
    Decode(Base64DecodeOpts),
}
#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    #[arg(short, long,value_parser = verify_file,default_value = "-")]
    pub input: String,

    #[arg(long,value_parser = parse_base64_format,default_value = "standard")]
    pub format: Base64Format,
}
#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long,value_parser = verify_file,default_value = "-")]
    pub input: String,
    #[arg(long,value_parser = parse_base64_format,default_value = "standard")]
    pub format: Base64Format,
}
```
> cargo run -- base64 encode -i Cargo.toml
> cargo run -- base64 encode --format urlsafe

## gen_pass
```rust
#[derive(Debug, Parser)]
pub struct GenPassOpts {
    // cargo run -- genpass -l 32
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub numbers: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}
```
> cargo run -- genpass -l 32
## sign and verify
```rust
#[derive(Debug, Parser)]
pub struct TextVerifyOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,
    #[arg(short, long, value_parser = verify_file)]
    pub key: String,
    #[arg(long, default_value = "blake3", value_parser = parse_format)]
    pub format: TextSignFormat,
    #[arg(long)]
    pub sig: String,
}
```
> cargo run -- text sign -k fixtures/blake3.txt
> cargo run -- text sign -k fixtures/ed25519.sk --format ed25519
> cargo run -- text verify -k fixtures/blake3.txt --sig key
> cargo run -- text verify -k fixtures/ed25519.pk --format ed25519 --sig key
