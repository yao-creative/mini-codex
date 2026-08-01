
    

pub struct ToolState;

pub struct ToolId(String);

pub struct Tool{
    pub id: ToolId,
}

pub struct ToolCallNodeId(String);

// instance of a tool call during a workflow
pub struct ToolCallNode{
    pub id: ToolCallNodeId,
    pub tool: Tool,
}

pub struct ScratchPad(String);

//DAG of tool calls in order
pub struct ExecutionGraph{
    pub tool_call_nodes: Vec<ToolCallNode>,
    pub tool_call_graph: Hashmap<ToolCallNodeId, Vec<ToolCallNode>>,
}

pub struct AgentState{
    scratchpad: String,
    tool_context: ToolState, //todo is this global context or local context? fix.
    execution_graph: ExecutionGraph,
}