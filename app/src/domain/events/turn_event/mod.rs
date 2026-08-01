enum TurnEvent {
    Delta(TextDelta),
    ToolStarted(ToolId),
    ToolFinished(ToolResult),
    StateUpdated(StatePatch),
    AssistantMessage(Message),
    Error(TurnError),
    Finished(TurnSummary),
}