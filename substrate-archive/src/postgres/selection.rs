use serde_json::Value;
use crate::entities::{Call, Event, ContractsEvent};
use crate::selection::{
    CallSelection, EventSelection, ContractsEventSelection, EthTransactSelection,
    GearMessageEnqueuedSelection, GearUserMessageSentSelection, EvmExecutedSelection,
};

const WILDCARD: &str = "*";

impl CallSelection {
    pub fn r#match(&self, call: &Call) -> bool {
        self.name == WILDCARD || self.name == call.name
    }
}

impl EventSelection {
    pub fn r#match(&self, event: &Event) -> bool {
        self.name == WILDCARD || self.name == event.name
    }
}

impl EthTransactSelection {
    pub fn r#match(&self, call: &Call) -> bool {
        if let Some(args) = &call.args {
            if let Some(transaction) = args.get("transaction") {
                let mut action = transaction.get("action");
                if action.is_none() {
                    if let Some(value) = transaction.get("value") {
                        action = value.get("action");
                    }
                }
                if let Some(action) = action {
                    if let Some(value) = action.get("value") {
                        if let Some(value) = value.as_str() {
                            return value == self.contract
                        }
                    }
                }
            }
        }
        false
    }
}

impl ContractsEventSelection {
    pub fn r#match(&self, event: &ContractsEvent) -> bool {
        if let Some(value) = event.data.get("contract") {
            if let Some(contract) = value.as_str() {
                return contract == self.contract
            }
        }
        false
    }
}

impl GearMessageEnqueuedSelection {
    pub fn r#match(&self, event: &Event) -> bool {
        if let Some(contract) = &event.contract {
            return contract == &self.program
        }
        false
    }
}

impl GearUserMessageSentSelection {
    pub fn r#match(&self, event: &Event) -> bool {
        if let Some(contract) = &event.contract {
            return contract == &self.program
        }
        false
    }
}

impl EvmExecutedSelection {
    pub fn r#match(&self, event: &Event) -> bool {
        if let Some(value) = &event.args {
            if let Some(value) = value.get("logs") {
                if let Some(logs) = value.as_array() {
                    for log in logs {
                        if let Some(value) = log.get("address") {
                            if let Some(address) = value.as_str() {
                                if address == self.contract && self.filter_match(log) {
                                    return true;
                                }
                            }
                        }
                    }
                }
            }
        }
        false
    }

    fn filter_match(&self, log: &Value) -> bool {
        let filter: Vec<_> = self.filter.iter().enumerate().collect();
        for (index, topics) in filter {
            if !self.topics_match(topics, log, index) {
                return false;
            }
        }
        true
    }

    fn topics_match(&self, topics: &Vec<String>, log: &Value, index: usize) -> bool {
        if topics.is_empty() {
            return true
        }

        if let Some(log_topic) = self.get_log_topic(log, index) {
            topics.iter().any(|topic| topic == &log_topic)
        } else {
            false
        }
    }

    fn get_log_topic(&self, log: &Value, index: usize) -> Option<String> {
        if let Some(value) = log.get("topics") {
            if let Some(value) = value.get(index) {
                if let Some(topic) = value.as_str() {
                    return Some(topic.to_string())
                }
            }
        }
        None
    }
}
