use super::selection::{
    CallSelection, EventSelection, EvmLogSelection, EthTransactSelection,
    ContractsEventSelection, GearMessageEnqueuedSelection, GearUserMessageSentSelection,
    EvmExecutedSelection,
};

pub struct BatchOptions {
    pub limit: i32,
    pub from_block: i32,
    pub to_block: Option<i32>,
    pub include_all_blocks: bool,
    pub call_selections: Vec<CallSelection>,
    pub event_selections: Vec<EventSelection>,
    pub evm_log_selections: Vec<EvmLogSelection>,
    pub eth_transact_selections: Vec<EthTransactSelection>,
    pub contracts_event_selections: Vec<ContractsEventSelection>,
    pub gear_message_enqueued_selections: Vec<GearMessageEnqueuedSelection>,
    pub gear_user_message_sent_selections: Vec<GearUserMessageSentSelection>,
    pub evm_executed_selections: Vec<EvmExecutedSelection>,
}
