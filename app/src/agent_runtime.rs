use crate::domain::{events::turn_event::TurnEvent, contexts::agent_runtime::AgentState};
use futures::stream::Stream;


pub struct AgentRuntime {
    planner: Planner,
    executor: Executor,
}


pub trait AgentRuntime{
    pub fn turn(&self, ctx: &mut AgentState, input: Input) -> Stream<TurnEvent>{
        //plan based on input (concat of user and system prompts) and context 

        
    }
}