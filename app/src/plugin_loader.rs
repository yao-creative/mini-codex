pub struct PluginLoader;

pub trait PluginLoaderTrait{
    fn load_plugin(&self, plugin: &Plugin) -> Result<()>{

    }
}
