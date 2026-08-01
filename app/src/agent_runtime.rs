
pub struct AgentRuntime {
    planner: Planner,

    executor: Executor,

}


pub trait AgentRuntime{
    pub fn turn(&self, ctx: &mut AgentContext) -> Stream<TurnEvent>{
        ...
    }
}