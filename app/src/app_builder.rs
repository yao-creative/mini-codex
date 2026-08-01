

//bootstrap builder. 
pub struct ApplicationBuilder <ConfigLoader, PluginLoader, SecretsLoader>{
    config_loader: ConfigLoader,
    plugin_loader: PluginLoader,
    secrets_loader: SecretsLoader,
}

pub trait ApplicationBuilderTrait{
    pub fn build(&self) -> Result<ApplicationRuntime>{
        // load config
        // initialize registries
        // wire dependencies
        // return ApplicationRuntime
        ...
    }
}
