# SigmaOS-libreoffice

LibreOffice integration and file format conversion CLI in Rust for SigmaOS.

## Features

- Document family and format detection (`odt`, `docx`, `ods`, `xlsx`, `csv`, `odp`, `pptx`, `pdf`, `html`)
- Compatibility validation between source and target formats
- Headless `soffice` conversion argument builder

## Usage

```bash
cargo run -- convert-args report.odt pdf /tmp/out
cargo test
```

## License

GPL-3.0-or-later
