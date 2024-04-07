use cosmwasm_std::{Addr, Empty, Uint128};
use cw20::Cw20Coin;
use cw20_base::msg::InstantiateMsg as Cw20InstantiateMsg;
use cw_multi_test::{App, Contract, ContractWrapper, Executor};
use packages::factory::{
    ExecuteMsg as FactoryExecuteMsg, InstantiateMsg as FactoryInstantiate, PoolInfo,
    QueryMsg as FactoryQueryMsg,
};
use packages::pair::{ExecuteMsg as PairExecuteMsg, QueryMsg as PairQueryMsg, Token, TokenInfo};
use packages::router::InstantiateMsg as RouterInstantiate;

fn mock_app() -> App {
    App::default()
}

#[allow(dead_code)]
fn router_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new_with_empty(
        router::contract::execute,
        router::contract::instantiate,
        router::contract::query,
    );
    // .with_reply(router::contract::reply);
    Box::new(contract)
}

#[allow(dead_code)]
fn factory_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new_with_empty(
        factory::contract::execute,
        factory::contract::instantiate,
        factory::contract::query,
    )
    .with_reply(factory::contract::reply);
    Box::new(contract)
}

#[allow(dead_code)]
fn pair_contract() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new_with_empty(
        pair::contract::execute,
        pair::contract::instantiate,
        pair::contract::query,
    );
    // .with_reply(pair::contract::reply);
    Box::new(contract)
}

#[allow(dead_code)]
fn mock_coin() -> Box<dyn Contract<Empty>> {
    let contract = ContractWrapper::new_with_empty(
        cw20_base::contract::execute,
        cw20_base::contract::instantiate,
        cw20_base::contract::query,
    );
    Box::new(contract)
}

#[derive(Debug)]
#[allow(dead_code)]
struct ContractInfo {
    token1_contract_addr: Addr,
    token2_contract_addr: Addr,
    factory_contract_addr: Addr,
    router_contract_addr: Addr,
}

fn initialize_contracts(app: &mut App) -> ContractInfo {
    let token1_code_id = app.store_code(mock_coin());
    println!("Token 1 code id: {}", token1_code_id);

    let token2_code_id = app.store_code(mock_coin());
    println!("Token 2 code id: {}", token2_code_id);

    let pair_code_id = app.store_code(pair_contract());
    println!("Pair code id: {}", pair_code_id);

    let factory_code_id = app.store_code(factory_contract());
    println!("Factory code id: {}", factory_code_id);

    let router_code_id = app.store_code(router_contract());
    println!("Router code id: {}", router_code_id);

    let token1_contract_addr = app
        .instantiate_contract(
            token1_code_id,
            Addr::unchecked("Sender"),
            &Cw20InstantiateMsg {
                name: "Mock Token 1".to_string(),
                symbol: "MTA".to_string(),
                decimals: 18u8,
                initial_balances: vec![Cw20Coin {
                    address: "user".to_string(),
                    amount: Uint128::from(1000000u128),
                }],
                mint: None,
                marketing: None,
            },
            &[],
            "Instantiate Mock coin 1",
            None,
        )
        .unwrap();
    println!("Mock Token 1 Address: {}", token1_contract_addr);

    let token2_contract_addr = app
        .instantiate_contract(
            token1_code_id,
            Addr::unchecked("Sender"),
            &Cw20InstantiateMsg {
                name: "Mock Token 2".to_string(),
                symbol: "MTB".to_string(),
                decimals: 18u8,
                initial_balances: vec![Cw20Coin {
                    address: "user".to_string(),
                    amount: Uint128::from(1000000u128),
                }],
                mint: None,
                marketing: None,
            },
            &[],
            "Instantiate Mock coin 2",
            None,
        )
        .unwrap();
    println!("Mock Token 2 Address: {}", token2_contract_addr);

    let factory_contract_addr = app
        .instantiate_contract(
            factory_code_id,
            Addr::unchecked("Sender"),
            &FactoryInstantiate { pair_code_id },
            &[],
            "Instantiate Factory",
            None,
        )
        .unwrap();
    println!("Factory contract addr: {}", factory_contract_addr);

    let router_contract_addr = app
        .instantiate_contract(
            router_code_id,
            Addr::unchecked("Sender"),
            &RouterInstantiate {
                factory_addr: factory_contract_addr.clone(),
            },
            &[],
            "Instantiate Router",
            None,
        )
        .unwrap();
    println!("Router contract addr: {}", router_contract_addr);

    ContractInfo {
        token1_contract_addr,
        token2_contract_addr,
        factory_contract_addr,
        router_contract_addr,
    }
}

#[test]
fn proper_initialization_test() {
    let mut app = mock_app();
    initialize_contracts(&mut app);
}

fn create_pair(
    app: &mut App,
    factory_contract_addr: Addr,
    token1_contract_addr: Addr,
    token2_contract_addr: Addr,
) -> String {
    let create_pair_res = app
        .execute_contract(
            Addr::unchecked("Sender"),
            factory_contract_addr,
            &FactoryExecuteMsg::CreateNewPair {
                asset_infos: [
                    TokenInfo::CW20Token {
                        contract_addr: token1_contract_addr,
                    },
                    TokenInfo::CW20Token {
                        contract_addr: token2_contract_addr,
                    },
                ],
            },
            &[],
        )
        .unwrap();
    let pair_address = create_pair_res.events[1].attributes[0].value.clone();
    return pair_address;
}

#[test]
fn create_pair_test() {
    let mut app = mock_app();
    let contract_info = initialize_contracts(&mut app);
    let pair_address = create_pair(
        &mut app,
        contract_info.factory_contract_addr,
        contract_info.token1_contract_addr,
        contract_info.token2_contract_addr,
    );

    println!("Pair Address {:?}", pair_address);
}

#[test]
fn query_pair_info_test() {
    let mut app = mock_app();
    let contract_info = initialize_contracts(&mut app);

    create_pair(
        &mut app,
        contract_info.factory_contract_addr.clone(),
        contract_info.token1_contract_addr.clone(),
        contract_info.token2_contract_addr.clone(),
    );

    let token_info_1 = TokenInfo::CW20Token {
        contract_addr: contract_info.token1_contract_addr.clone(),
    };

    let token_info_2 = TokenInfo::CW20Token {
        contract_addr: contract_info.token2_contract_addr.clone(),
    };

    let query_res: PoolInfo = app
        .wrap()
        .query_wasm_smart(
            contract_info.factory_contract_addr,
            &FactoryQueryMsg::Pair {
                asset_infos: [token_info_1.clone(), token_info_2.clone()],
            },
        )
        .unwrap();

    println!(
        "Query Pair Reponse: {:?} {:?}\n",
        query_res, query_res.pair_addr
    );
}

#[test]
fn add_liquidity_test() {
    let mut app = mock_app();
    let contract_info = initialize_contracts(&mut app);

    let token_info_1 = TokenInfo::CW20Token {
        contract_addr: contract_info.token1_contract_addr.clone(),
    };

    let token_info_2 = TokenInfo::CW20Token {
        contract_addr: contract_info.token2_contract_addr.clone(),
    };

    let pair_contract_addr = create_pair(
        &mut app,
        contract_info.factory_contract_addr.clone(),
        contract_info.token1_contract_addr.clone(),
        contract_info.token2_contract_addr.clone(),
    );

    app.execute_contract(
        Addr::unchecked("user"),
        contract_info.token1_contract_addr.clone(),
        &cw20::Cw20ExecuteMsg::IncreaseAllowance {
            spender: pair_contract_addr.clone(),
            amount: Uint128::from(200u128),
            expires: None,
        },
        &[],
    )
    .unwrap();
    println!("Increased Allowance for Token1 \n",);

    app.execute_contract(
        Addr::unchecked("user"),
        contract_info.token2_contract_addr.clone(),
        &cw20::Cw20ExecuteMsg::IncreaseAllowance {
            spender: pair_contract_addr.clone(),
            amount: Uint128::from(100u128),
            expires: None,
        },
        &[],
    )
    .unwrap();

    println!("Increased Allowance for Token2 \n",);

    let get_liquidity_amt: Uint128 = app
        .wrap()
        .query_wasm_smart(
            contract_info.factory_contract_addr,
            &PairQueryMsg::GetLpAmount {
                asset_infos: [token_info_1.clone(), token_info_2.clone()],
            },
        )
        .unwrap();

    println!("Getting liquidity amount {:?}", get_liquidity_amt);

    let add_liquidity_res = app
        .execute_contract(
            Addr::unchecked("user"),
            Addr::unchecked(pair_contract_addr.clone()),
            &PairExecuteMsg::AddLiquidity {
                assets: [
                    Token {
                        info: token_info_1.clone(),
                        amount: Uint128::from(100u128),
                    },
                    Token {
                        info: token_info_2.clone(),
                        amount: Uint128::from(100u128),
                    },
                ],
                min_liquidity_amt: Uint128::from(1u128),
            },
            &[],
        )
        .unwrap();

    println!("Add liquidity Response: {:?}\n", add_liquidity_res);
}

#[test]
fn swap_token_test() {
    let mut app = mock_app();
    let contract_info = initialize_contracts(&mut app);

    let token_info_1 = TokenInfo::CW20Token {
        contract_addr: contract_info.token1_contract_addr.clone(),
    };

    let token_info_2 = TokenInfo::CW20Token {
        contract_addr: contract_info.token2_contract_addr.clone(),
    };

    let pair_contract_addr = create_pair(
        &mut app,
        contract_info.factory_contract_addr.clone(),
        contract_info.token1_contract_addr.clone(),
        contract_info.token2_contract_addr.clone(),
    );

    let increase_allowance_token1 = app
        .execute_contract(
            Addr::unchecked("user"),
            contract_info.token1_contract_addr.clone(),
            &cw20::Cw20ExecuteMsg::IncreaseAllowance {
                spender: pair_contract_addr.clone(),
                amount: Uint128::from(200u128),
                expires: None,
            },
            &[],
        )
        .unwrap();
    println!(
        "Increased Allowance for Token1: {:?}\n",
        increase_allowance_token1
    );

    let increase_allowance_token2 = app
        .execute_contract(
            Addr::unchecked("user"),
            contract_info.token2_contract_addr.clone(),
            &cw20::Cw20ExecuteMsg::IncreaseAllowance {
                spender: pair_contract_addr.clone(),
                amount: Uint128::from(100u128),
                expires: None,
            },
            &[],
        )
        .unwrap();

    println!(
        "Increased Allowance for Token2: {:?}\n",
        increase_allowance_token2
    );

    app.execute_contract(
        Addr::unchecked("user"),
        Addr::unchecked(pair_contract_addr.clone()),
        &PairExecuteMsg::AddLiquidity {
            assets: [
                Token {
                    info: token_info_1.clone(),
                    amount: Uint128::from(100u128),
                },
                Token {
                    info: token_info_2.clone(),
                    amount: Uint128::from(100u128),
                },
            ],
            min_liquidity_amt: Uint128::from(1u128),
        },
        &[],
    )
    .unwrap();

    let swap_res = app
        .execute_contract(
            Addr::unchecked("user"),
            Addr::unchecked(pair_contract_addr.clone()),
            &PairExecuteMsg::SwapAsset {
                from_token: token_info_1.clone(),
                to_token: token_info_2.clone(),
                amount_in: 20u128,
                min_amount_out: 5u128,
            },
            &[],
        )
        .unwrap();

    println!("Swap Response: {:?}\n", swap_res);
}

#[test]
fn withdraw_liquidity() {
    let mut app = mock_app();
    let contract_info = initialize_contracts(&mut app);

    let token_info_1 = TokenInfo::CW20Token {
        contract_addr: contract_info.token1_contract_addr.clone(),
    };

    let token_info_2 = TokenInfo::CW20Token {
        contract_addr: contract_info.token2_contract_addr.clone(),
    };

    let pair_contract_addr = create_pair(
        &mut app,
        contract_info.factory_contract_addr.clone(),
        contract_info.token1_contract_addr.clone(),
        contract_info.token2_contract_addr.clone(),
    );

    app.execute_contract(
        Addr::unchecked("user"),
        contract_info.token1_contract_addr.clone(),
        &cw20::Cw20ExecuteMsg::IncreaseAllowance {
            spender: pair_contract_addr.clone(),
            amount: Uint128::from(200u128),
            expires: None,
        },
        &[],
    )
    .unwrap();
    println!("Increased Allowance for Token1 \n",);

    app.execute_contract(
        Addr::unchecked("user"),
        contract_info.token2_contract_addr.clone(),
        &cw20::Cw20ExecuteMsg::IncreaseAllowance {
            spender: pair_contract_addr.clone(),
            amount: Uint128::from(100u128),
            expires: None,
        },
        &[],
    )
    .unwrap();

    println!("Increased Allowance for Token2\n",);

    let add_liquidity_res = app
        .execute_contract(
            Addr::unchecked("user"),
            Addr::unchecked(pair_contract_addr.clone()),
            &PairExecuteMsg::AddLiquidity {
                assets: [
                    Token {
                        info: token_info_1.clone(),
                        amount: Uint128::from(100u128),
                    },
                    Token {
                        info: token_info_2.clone(),
                        amount: Uint128::from(100u128),
                    },
                ],
                min_liquidity_amt: Uint128::from(1u128),
            },
            &[],
        )
        .unwrap();

    println!("Add liquidity Response: {:?}\n", add_liquidity_res);

    let withdraw_res = app.execute_contract(
        Addr::unchecked("user"),
        Addr::unchecked(pair_contract_addr.clone()),
        &PairExecuteMsg::RemoveLiquidity {
            lp_token: Token {
                info: token_info_1.clone(),
                amount: Uint128::from(20u128),
            },
        },
        &[],
    );

    println!("Withdraw Res: {:?}\n", withdraw_res);
}
