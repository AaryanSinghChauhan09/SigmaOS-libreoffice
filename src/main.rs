use anyhow::Result;
use clap::Parser;
use libreoffice::libreoffice;

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    no_integration: bool,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = libreoffice::Config { 
        desktop_integration: !cli.no_integration, 
        ..Default::default() 
    };
    let mut component = libreoffice::new(config);
    
    component.initialize()?;
    component.launch()?;
    
    println!("SigmaOS-libreoffice ready");
    Ok(())
}
