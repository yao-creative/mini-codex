pub struct AppRuntime {
    scheduler: Scheduler,
    telemetry: Telemetry,
}

// todo: separate runtime states from runtime behavior
pub struct AppRuntimeTrait{
    fn run(&self) -> Result<AppRuntimeResult, AppRuntimeError>{
        //startup with session asking for log in if needed.
        ...
    }
}

pub impl AppRuntime{
    
}