use ::libreoffice::{libreoffice, Config};

fn main() -> anyhow::Result<()> {
    let config = Config::default();
    let mut component = libreoffice::new(config);
    component.initialize().map_err(|e| anyhow::anyhow!("{}", e))?;
    println!("SigmaOS-libreoffice operational");
    Ok(())
}
