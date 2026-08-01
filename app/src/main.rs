
fn main() -> Result<()> {
    let app = ApplicationBuilder::new()
        .load_config()?
        .load_plugins()?
        .build()?;

    app.run()
}
