pub struct SecretsLoader;
pub trait SecretsLoaderTrait{
    fn load_secrets(&self, secrets: &Secrets) -> Result<()>{

    }
}
