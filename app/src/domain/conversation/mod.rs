pub struct Conversation{
    id: ConversationId,
    history: MessageHistory,
    attachments: Vec<Attachment>,
    memories: Vec<MemoryReference>,
}
