


pub struct ConversationId(pub String);

use std::collections::VecDeque;
use std::time::SystemTime;

pub enum Actor {
    User,
    Agent,
    // Extend as needed for other actor roles
}

pub struct MessageId(pub String);

pub struct Message {
    pub id: MessageId,
    pub actor: Actor,
    pub text: String,
    pub created_at: SystemTime,
}

pub struct MessageHistoryState {
    pub messages: VecDeque<Message>,
}

pub struct ConversationState {
    id: ConversationId,
    history: MessageHistoryState,
    agent: AgentState,
}