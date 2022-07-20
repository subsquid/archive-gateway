use serde_json::json;
use common::{launch_gateway, Client};

mod common;

#[actix_web::test]
async fn test_parent_call_loaded() {
    launch_gateway();
    let client = Client::new();
    let selections_data = [
        json!({"_all": true}),
        json!({"parent": {"_all": true}})
    ];
    for selection_data in selections_data {
        let batch = client.batch(json!({
            "limit": 1,
            "calls": [{
                "name": "Balances.transfer",
                "data": {"call": selection_data},
            }]
        })).await;
        let requested_call = batch.calls.iter()
            .find(|call| call.id == "0000650677-000003-0f08a-000001".to_string());
        let parent_call = batch.calls.iter()
            .find(|call| call.id == "0000650677-000003-0f08a".to_string());
        assert!(requested_call.is_some());
        assert!(parent_call.is_some());
    }
}

#[actix_web::test]
async fn test_parent_call_skipped() {
    launch_gateway();
    let client = Client::new();
    let batch = client.batch(json!({
        "limit": 1,
        "calls": [{
            "name": "Balances.transfer",
            "data": {
                "call": {
                    "parent": {"_all": false}
                },
                "extrinsic": {"_all": false}
            }
        }]
    })).await;
    let requested_call = batch.calls.iter()
        .find(|call| call.id == "0000650677-000003-0f08a-000001".to_string());
    let parent_call = batch.calls.iter()
        .find(|call| call.id == "0000650677-000003-0f08a".to_string());
    assert!(requested_call.is_some());
    assert!(parent_call.is_none());
}

#[actix_web::test]
async fn test_event_loads_related_call() {
    launch_gateway();
    let client = Client::new();
    let selections = [
        json!({"name": "Contracts.ContractEmitted"}),
        json!({
            "name": "Contracts.ContractEmitted",
            "data": {"event": {"_all": true}},
        }),
    ];
    for selection in selections {
        let batch = client.batch(json!({
            "limit": 1,
            "events": json!([selection])
        })).await;
        let call = &batch.calls[0];
        assert!(call.id == "0000000734-000001-251d1".to_string());
    }
}

#[actix_web::test]
async fn test_evm_log_has_tx_hash() {
    launch_gateway();
    let client = Client::new();
    let batch = client.batch(json!({
        "limit": 1,
        "evmLogs": [{
            "contract": "0xb654611f84a8dc429ba3cb4fda9fad236c505a1a",
            "filter": [["0xddf252ad1be2c89b69c2b068fc378daa952ba7f163c4a11628f55a4df523b3ef"]],
            "data": {
                "event": {
                    "evmTxHash": true,
                }
            }
        }]
    })).await;
    let log = &batch.events[0];
    assert!(log.id == "0000569006-000084-5e412".to_string());
    assert!(log.evmTxHash == Some("0x8eafe131eee90e0dfb07d6df46b1aea737834936968da31f807af566a59148b9".to_string()));
}

#[actix_web::test]
async fn test_ethereum_transactions() {
    launch_gateway();
    let client = Client::new();
    let batch = client.batch(json!({
        "limit": 1,
        "ethereumTransactions": [{
            "contract": "0xb654611f84a8dc429ba3cb4fda9fad236c505a1a"
        }]
    })).await;
    let call = &batch.calls[0];
    assert!(call.id == "0000569006-000018-5e412".to_string());
}

#[actix_web::test]
async fn test_contracts_events() {
    launch_gateway();
    let client = Client::new();
    let batch = client.batch(json!({
        "limit": 1,
        "contractsEvents": [{
            "contract": "0xf98402623dbe32d22b67e0a25136b763615c14fd4201b1aac8832ec52aa64d10",
            "data": {
                "event": {
                    "_all": true
                }
            }
        }]
    })).await;
    let event = &batch.events[0];
    assert!(event.id == "0000000734-000004-251d1".to_string());
}

#[actix_web::test]
async fn test_wildcard_search() {
    launch_gateway();
    let client = Client::new();
    let batch = client.batch(json!({
        "limit": 1,
        "fromBlock": 734,
        "calls": [{"name": "*"}],
        "events": [{"name": "*"}],
    })).await;
    let event = &batch.events[0];
    assert!(event.id == "0000000734-000004-251d1".to_string());
    assert!(event.name == "Contracts.ContractEmitted".to_string());
    let call = &batch.calls[0];
    assert!(call.id == "0000000734-000001-251d1".to_string());
    assert!(call.name == "Contracts.call".to_string());
}

#[actix_web::test]
async fn test_contracts_wildcard_search() {
    launch_gateway();
    let client = Client::new();
    let batch = client.batch(json!({
        "limit": 1,
        "contractsEvents": [{"contract": "*"}],
    })).await;
    let event = &batch.events[0];
    assert!(event.id == "0000000734-000004-251d1".to_string());
    assert!(event.name == "Contracts.ContractEmitted".to_string());
}

#[actix_web::test]
async fn test_evm_wildcard_search() {
    launch_gateway();
    let client = Client::new();
    let batch = client.batch(json!({
        "limit": 1,
        "evmLogs": [{"contract": "*"}],
    })).await;
    let event = &batch.events[0];
    assert!(event.id == "0000569006-000084-5e412".to_string());
    assert!(event.name == "EVM.Log".to_string());
}

#[actix_web::test]
async fn test_gear_messages_enqueued() {
    launch_gateway();
    let client = Client::new();
    let batch = client.batch(json!({
        "limit": 1,
        "gearMessagesEnqueued": [{"program": "0x5466a0b28225bcc8e33f6bdde7a95f5fcf81bfd94d70ca099d0a8bae230dbaa1"}],
    })).await;
    let event = &batch.events[0];
    assert!(event.id == "0000000006-000003-7ec94".to_string());
    assert!(event.name == "Gear.MessageEnqueued");
}

#[actix_web::test]
async fn test_gear_user_sent_messages() {
    launch_gateway();
    let client = Client::new();
    let batch = client.batch(json!({
        "limit": 1,
        "gearUserMessagesSent": [{"program": "0x5466a0b28225bcc8e33f6bdde7a95f5fcf81bfd94d70ca099d0a8bae230dbaa1"}],
    })).await;
    let event = &batch.events[0];
    assert!(event.id == "0000000006-000007-7ec94".to_string());
    assert!(event.name == "Gear.UserMessageSent");
}
