use imports::{MxscPath, TestAddress, TestSCAddress, TestTokenIdentifier};
use multiversx_sc_scenario::*;

const OWNER_ADDRESS: TestAddress =
    TestAddress::new("erd14pytqekzvghdl9frcdu9pyt2c34wjtw4hq4aequntmp54whfdzsqsg7hft");
const SECOND_USER: TestAddress =
    TestAddress::new("erd18tl5dm72ppkzmx5kvxjlnclrd7wa349r2ytutx60ugqhq5gnl66s5046zd");
const _SC_ADDRESS: TestSCAddress =
    TestSCAddress::new("erd1qqqqqqqqqqqqqpgqrhzm5tlnqgyxmc0suqfcfwzn8fxcfdg4dzsqysc3tt");
const _CODE_PATH: MxscPath = MxscPath::new("../output/on-chain-claim.mxsc.json");
const TOKEN: TestTokenIdentifier = TestTokenIdentifier::new("VLAD-6bde05");
const _INVALID_TOKEN: TestTokenIdentifier = TestTokenIdentifier::new("12sasdf");

fn world() -> ScenarioWorld {
    let mut blockchain = ScenarioWorld::new();

    blockchain
        .account(OWNER_ADDRESS)
        .balance(100)
        .esdt_balance(TOKEN, 100);
    blockchain
        .account(SECOND_USER)
        .balance(1000)
        .esdt_balance(TOKEN, 1000);
    blockchain.current_block().block_epoch(1);

    blockchain.register_contract(
        "mxsc:output/on-chain-claim.mxsc.json",
        on_chain_claim::ContractBuilder,
    );
    blockchain
}

#[test]
fn trace_1_rs() {
    world().run("scenarios/trace1.scen.json");
}

#[test]
fn trace_13_rs() {
    world().run("scenarios/trace13.scen.json");
}

#[test]
fn trace_2_rs() {
    world().run("scenarios/trace2.scen.json");
}

#[test]
fn trace_3_rs() {
    world().run("scenarios/trace3.scen.json");
}

#[test]
fn trace_5_rs() {
    world().run("scenarios/trace5.scen.json");
}

#[test]
fn trace_7_rs() {
    world().run("scenarios/trace7.scen.json");
}

#[test]
fn trace_8_rs() {
    world().run("scenarios/trace8.scen.json");
}
