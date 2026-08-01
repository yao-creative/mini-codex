
    

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub struct ActionId(usize);


impl ActionId {
    pub fn new(id: usize) -> Self {
        ActionId(id)
    }
    
    pub fn as_usize(&self) -> usize {
        self.0
    }
}

pub struct ScratchPad(String);

pub struct ToolContext;

pub struct ExecutionGraph{
    pub tools: Vec<Tool>
    pub tool_call_graph: Hashmap<ToolId
}

pub struct AgentContext{
    scratchpad: String,

    tool_context: ToolContext,

    execution_graph: ExecutionGraph,

}