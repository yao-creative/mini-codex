pub struct AppRuntime {
    config: Config,
    model_registry: Arc<ModelRegistry>,
    tool_registry: Arc<ToolRegistry>,
    memory_backend: Arc<MemoryBackend>,
    database: Arc<Database>,
    scheduler: Scheduler,
    telemetry: Telemetry,
}

// todo: separate runtime states from runtime behavior