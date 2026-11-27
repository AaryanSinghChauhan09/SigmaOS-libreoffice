use sigmaos_libreoffice::{ConversionJob, DocumentFormat};
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();
    let cmd = args.get(1).map(String::as_str).unwrap_or("help");

    match cmd {
        "convert-args" => {
            let input = args.get(2).map(String::as_str).unwrap_or("document.odt");
            let fmt_str = args.get(3).map(String::as_str).unwrap_or("pdf");
            let out_dir = args.get(4).map(String::as_str).unwrap_or(".");

            let fmt = DocumentFormat::from_extension(fmt_str).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                process::exit(1);
            });
            let job = ConversionJob::new(input, out_dir, fmt).unwrap_or_else(|e| {
                eprintln!("Error: {}", e);
                process::exit(1);
            });
            println!("soffice {}", job.to_soffice_args().join(" "));
        }
        _ => {
            println!("sigmaos-libreoffice - LibreOffice Conversion CLI");
            println!("Usage: sigmaos-libreoffice convert-args <input> <format> [outdir]");
        }
    }
}
