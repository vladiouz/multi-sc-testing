use multiversx_sc_scenario::imports::*;

mod proxy;

const OWNER_ADDRESS: TestAddress =
    TestAddress::new("erd14pytqekzvghdl9frcdu9pyt2c34wjtw4hq4aequntmp54whfdzsqsg7hft");
const SECOND_USER: TestAddress =
    TestAddress::new("erd18tl5dm72ppkzmx5kvxjlnclrd7wa349r2ytutx60ugqhq5gnl66s5046zd");
const SC_ADDRESS: TestSCAddress =
    TestSCAddress::new("erd1qqqqqqqqqqqqqpgqrhzm5tlnqgyxmc0suqfcfwzn8fxcfdg4dzsqysc3tt");
const CODE_PATH: MxscPath = MxscPath::new("output/on-chain-claim.mxsc.json");
const TOKEN: TestTokenIdentifier = TestTokenIdentifier::new("VLAD-6bde05");
const INVALID_TOKEN: TestTokenIdentifier = TestTokenIdentifier::new("12sasdf");

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

    blockchain.register_contract(CODE_PATH, on_chain_claim::ContractBuilder);
    blockchain
}

#[test]
fn on_chain_claim_blackbox_init() {
    let mut world = world();

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .init(TOKEN, 0u64)
        .code(CODE_PATH)
        .code_metadata(CodeMetadata::PAYABLE)
        .returns(ReturnsNewAddress)
        .new_address(SC_ADDRESS)
        .run();

    assert_eq!(new_address, SC_ADDRESS);
}

#[test]
fn on_chain_claim_blackbox_init_invalid_token_id() {
    let mut world = world();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .init(INVALID_TOKEN, 0u64)
        .code(CODE_PATH)
        .code_metadata(CodeMetadata::PAYABLE)
        .returns(ExpectError(4, "Invalid token ID"))
        .new_address(SC_ADDRESS)
        .run();
}

#[test]
fn on_chain_claim_double_claim() {
    let mut world = world();

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .init(TOKEN, 0u64)
        .code(CODE_PATH)
        .code_metadata(CodeMetadata::PAYABLE)
        .returns(ReturnsNewAddress)
        .new_address(SC_ADDRESS)
        .run();

    assert_eq!(new_address, SC_ADDRESS);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .claim()
        .returns(ReturnsResult)
        .run();

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .claim()
        .returns(ExpectError(4, "epoch already claimed"))
        .run();
}

#[test]
fn on_chain_claim_late_claim() {
    let mut world = world();

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .init(TOKEN, 0u64)
        .code(CODE_PATH)
        .code_metadata(CodeMetadata::PAYABLE)
        .returns(ReturnsNewAddress)
        .new_address(SC_ADDRESS)
        .run();

    assert_eq!(new_address, SC_ADDRESS);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .claim()
        .returns(ReturnsResult)
        .run();

    world.current_block().block_epoch(3);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .claim()
        .returns(ReturnsResult)
        .run();

    let address_info = world
        .query()
        .to(SC_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .get_address_info(OWNER_ADDRESS)
        .returns(ReturnsResult)
        .run();

    assert_eq!(address_info.best_streak, 1);
    assert_eq!(address_info.current_streak, 1);
    assert_eq!(address_info.last_epoch_claimed, 3);
    assert_eq!(address_info.total_epochs_claimed, 2);
}

#[test]
fn on_chain_claim_wrong_shard_claim() {
    let mut world = world();

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .init(TOKEN, 0u64)
        .code(CODE_PATH)
        .code_metadata(CodeMetadata::PAYABLE)
        .returns(ReturnsNewAddress)
        .new_address(SC_ADDRESS)
        .run();

    assert_eq!(new_address, SC_ADDRESS);

    world
        .tx()
        .from(SECOND_USER)
        .to(SC_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .claim()
        .returns(ExpectError(4, "wrong shard"))
        .run();
}

#[test]
fn on_chain_claim_claim_happy_path() {
    let mut world = world();

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .init(TOKEN, 0u64)
        .code(CODE_PATH)
        .code_metadata(CodeMetadata::PAYABLE)
        .returns(ReturnsNewAddress)
        .new_address(SC_ADDRESS)
        .run();

    assert_eq!(new_address, SC_ADDRESS);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .claim()
        .returns(ReturnsResult)
        .run();

    world.current_block().block_epoch(2);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .claim()
        .returns(ReturnsResult)
        .run();

    let address_info = world
        .query()
        .to(SC_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .get_address_info(OWNER_ADDRESS)
        .returns(ReturnsResult)
        .run();

    assert_eq!(address_info.best_streak, 2);
    assert_eq!(address_info.current_streak, 2);
    assert_eq!(address_info.last_epoch_claimed, 2);
    assert_eq!(address_info.total_epochs_claimed, 2);
}

#[test]
fn on_chain_claim_claim_and_repair_bad_amount() {
    let mut world = world();

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .init(TOKEN, 0u64)
        .code(CODE_PATH)
        .code_metadata(CodeMetadata::PAYABLE)
        .returns(ReturnsNewAddress)
        .new_address(SC_ADDRESS)
        .run();

    assert_eq!(new_address, SC_ADDRESS);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .claim_and_repair()
        .payment((
            TokenIdentifier::from(TOKEN),
            0,
            BigUint::<StaticApi>::from(2u128),
        ))
        .returns(ExpectError(4, "Bad payment token/amount"))
        .run();
}

#[test]
fn on_chain_claim_claim_and_repair_wrong_shard() {
    let mut world = world();

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .init(TOKEN, 0u64)
        .code(CODE_PATH)
        .code_metadata(CodeMetadata::PAYABLE)
        .returns(ReturnsNewAddress)
        .new_address(SC_ADDRESS)
        .run();

    assert_eq!(new_address, SC_ADDRESS);

    world
        .tx()
        .from(SECOND_USER)
        .to(SC_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .claim_and_repair()
        .payment((
            TokenIdentifier::from(TOKEN),
            0,
            BigUint::<StaticApi>::from(1u128),
        ))
        .returns(ExpectError(4, "wrong shard"))
        .run();
}

#[test]
fn on_chain_claim_claim_and_repair_non_burnable_token() {
    let mut world = world();

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .init(TOKEN, 0u64)
        .code(CODE_PATH)
        .code_metadata(CodeMetadata::PAYABLE)
        .returns(ReturnsNewAddress)
        .new_address(SC_ADDRESS)
        .run();

    assert_eq!(new_address, SC_ADDRESS);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .claim()
        .returns(ReturnsResult)
        .run();

    world.current_block().block_epoch(4);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .claim_and_repair()
        .payment((
            TokenIdentifier::from(TOKEN),
            0,
            BigUint::<StaticApi>::from(1u128),
        ))
        .returns(ExpectError(10, "action is not allowed"))
        .run();
}

#[test]
fn on_chain_claim_claim_and_repair_happy_path() {
    let mut world = world();

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .init(TOKEN, 0u64)
        .code(CODE_PATH)
        .code_metadata(CodeMetadata::PAYABLE)
        .returns(ReturnsNewAddress)
        .new_address(SC_ADDRESS)
        .run();

    assert_eq!(new_address, SC_ADDRESS);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .claim()
        .returns(ReturnsResult)
        .run();

    world.current_block().block_epoch(4);
    world.set_esdt_local_roles(OWNER_ADDRESS, b"VLAD-6bde05", &[EsdtLocalRole::Burn]);
    world.set_esdt_local_roles(SC_ADDRESS, b"VLAD-6bde05", &[EsdtLocalRole::Burn]);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .claim_and_repair()
        .payment((
            TokenIdentifier::from(TOKEN),
            0,
            BigUint::<StaticApi>::from(1u128),
        ))
        .returns(ReturnsResult)
        .run();

    let address_info = world
        .query()
        .to(SC_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .get_address_info(OWNER_ADDRESS)
        .returns(ReturnsResult)
        .run();

    assert_ne!(address_info.best_streak, 0);
}

#[test]
fn on_chain_claim_update_state_happy_path() {
    let mut world = world();

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .init(TOKEN, 0u64)
        .code(CODE_PATH)
        .code_metadata(CodeMetadata::PAYABLE)
        .returns(ReturnsNewAddress)
        .new_address(SC_ADDRESS)
        .run();

    assert_eq!(new_address, SC_ADDRESS);

    world
        .tx()
        .from(OWNER_ADDRESS)
        .to(SC_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .update_state(
            ManagedAddress::from_address(&OWNER_ADDRESS.to_address()),
            1u64,
            2u64,
            20u64,
            40u64,
        )
        .returns(ReturnsResult)
        .run();

    let address_info = world
        .query()
        .to(SC_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .get_address_info(OWNER_ADDRESS)
        .returns(ReturnsResult)
        .run();

    assert_eq!(address_info.best_streak, 40);
    assert_eq!(address_info.current_streak, 1);
    assert_eq!(address_info.last_epoch_claimed, 2);
    assert_eq!(address_info.total_epochs_claimed, 20);
}

#[test]
fn on_chain_claim_update_state_non_admin() {
    let mut world = world();

    let new_address = world
        .tx()
        .from(OWNER_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .init(TOKEN, 0u64)
        .code(CODE_PATH)
        .code_metadata(CodeMetadata::PAYABLE)
        .returns(ReturnsNewAddress)
        .new_address(SC_ADDRESS)
        .run();

    assert_eq!(new_address, SC_ADDRESS);

    world
        .tx()
        .from(SECOND_USER)
        .to(SC_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .update_state(
            ManagedAddress::from_address(&OWNER_ADDRESS.to_address()),
            1u64,
            2u64,
            20u64,
            40u64,
        )
        .returns(ExpectError(4, "Endpoint can only be called by admins"))
        .run();
}

#[test]
fn on_chain_claim_update_state_wrong_shard() {
    let mut world = world();

    let new_address = world
        .tx()
        .from(SECOND_USER)
        .typed(proxy::OnChainClaimContractProxy)
        .init(TOKEN, 0u64)
        .code(CODE_PATH)
        .code_metadata(CodeMetadata::PAYABLE)
        .returns(ReturnsNewAddress)
        .new_address(SC_ADDRESS)
        .run();

    assert_eq!(new_address, SC_ADDRESS);

    world
        .tx()
        .from(SECOND_USER)
        .to(SC_ADDRESS)
        .typed(proxy::OnChainClaimContractProxy)
        .update_state(
            ManagedAddress::from_address(&SECOND_USER.to_address()),
            1u64,
            2u64,
            20u64,
            40u64,
        )
        .returns(ExpectError(4, "wrong shard"))
        .run();
}
