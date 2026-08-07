
pub struct ThreadManagerState{
    config: Config,
    model_registry: Arc<ModelRegistry>,
    tool_registry: Arc<ToolRegistry>,
    memory_backend: Arc<MemoryBackend>,
    database: Arc<Database>,
}