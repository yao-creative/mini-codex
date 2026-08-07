

pub struct ConfigLoader;

pub trait ConfigLoaderTrait{
    fn load_config(&self, config: &Config) -> Result<()>{

    }
}


