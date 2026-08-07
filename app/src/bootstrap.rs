pub struct Bootstrap;

trait BootstrapTrait {
    fn build(self, args: &Vec<String>) -> anyhow::Result<Host> {
    }
}



pub impl BootstrapTrait for Bootstrap {
    fn build(args: CliArgs) -> anyhow::Result<Host> {
        ConfigBuilder::build(args)
            .and_then(|config| {
                Storage::new(&config.database).map(|storage| (config, storage))
            })
            .map(|(config, storage)| {
                let auth = Authenticator::new(config.auth);
                let app = ApplicationRuntime::new(storage, auth);

                Host {
                    app,
                    tui: TuiRuntime::new(),
                    engine: Engine::new(),
                }
            })
    }
}