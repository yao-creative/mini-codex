use crate::domain::states::conversation::{MessageHistoryState};
use crate::domain::states::conversation::err::{TruncateError,AppendError, IterError};


pub struct MessageHistoryController;

pub trait MessageHistoryController {

    fn append(
        &self,
        state: &mut MessageHistoryState,
        message: Message,
    ) -> Result<(), AppendError>;

    fn truncate(
        &self,
        state: &mut MessageHistoryState,
        budget: TokenBudget,
    ) -> Result<(), TruncateError>;

    fn iter<'a>(
        &self,
        state: &'a MessageHistoryState,
    ) -> Result<std::slice::Iter<'a, Message>, IterError>;
}


pub impl MessageHistoryControllerTrait for MessageHistoryController {
    fn append(&self, state: &mut MessageHistoryState, message: Message) -> Result<(), AppendError> {
        state.messages.push(message);
        Ok()
    }

    fn truncate(&self, state: &mut MessageHistoryState, budget: TokenBudget) -> Result<(), TruncateError>  {  
        if budget.is_enough() {
            Ok()
        } else {
            Err(TruncateError::NotEnoughBudget)
        }
     }

    fn iter(&self, state: &MessageHistoryState) -> std::slice::Iter<'a, Message> {  
        state.messages.iter()
    }
}