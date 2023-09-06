use cosmwasm_std::{Empty, Addr, Uint128};
use cw20::{Cw20Coin, AllowanceResponse, BalanceResponse};
use cw_multi_test::{App, Contract, ContractWrapper, Executor};
use cw20_base::msg::{InstantiateMsg as Cw20InstantiateMsg, QueryMsg as Cw20QueryMsg};
use factory::state::PoolInfo;
use packages::pair::{TokenInfo, ExecuteMsg as PairExecuteMsg, Token, QueryMsg as PairQueryMsg};
use packages::factory::{InstantiateMsg as FactoryInstantiate, ExecuteMsg as FactoryExecuteMsg, QueryMsg as FactoryQueryMsg};
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
        cw20_base::contract::query
    );
    Box::new(contract)
}

#[test]
fn integration_test() {

    let mut app = mock_app();

    //Store code

    let token1_code_id = app.store_code(mock_coin());
    println!("Token 1 code id: {}", token1_code_id);

    let token2_code_id = app.store_code(mock_coin());
    println!("Token 1 code id: {}", token2_code_id);

    let pair_code_id = app.store_code(pair_contract());
    println!("Pair code id: {}", pair_code_id);

    let factory_code_id = app.store_code(factory_contract());
    println!("Factory code id: {}", factory_code_id);

    let router_code_id = app.store_code(router_contract());
    println!("Router code id: {}", router_code_id);

    //Instantiate Contract

    let token1_contract_addr = app.instantiate_contract(
        token1_code_id, 
        Addr::unchecked("Sender"), 
        &Cw20InstantiateMsg{
            name: "Mock Token 1".to_string(),
            symbol: "MTA".to_string(),
            decimals: 18u8,
            initial_balances: vec![
                Cw20Coin {
                    address: "user".to_string(),
                    amount: Uint128::from(1000000u128),
                },
            ],
            // mint: Some(MinterResponse {
            //     minter: "Sender".to_string(),
            //     cap: Some(Uint128::from(1000000000u128)),
            // }),
            mint: None,
            marketing: None,
        }, 
        &[], 
        "Instantiate Mock coin 1", 
        None,
    ).unwrap();
    println!("Mock Token 1 Address: {}", token1_contract_addr);

    let token2_contract_addr = app.instantiate_contract(
        token1_code_id, 
        Addr::unchecked("Sender"), 
        &Cw20InstantiateMsg{
            name: "Mock Token 2".to_string(),
            symbol: "MTB".to_string(),
            decimals: 18u8,
            initial_balances: vec![
                Cw20Coin {
                    address: "user".to_string(),
                    amount: Uint128::from(1000000u128),
                },
            ],
            // mint: Some(MinterResponse {
            //     minter: "Sender".to_string(),
            //     cap: Some(Uint128::from(1000000000u128)),
            // }),
            mint: None,
            marketing: None,
        }, 
        &[], 
        "Instantiate Mock coin 2", 
        None,
    ).unwrap();
    println!("Mock Token 2 Address: {}", token2_contract_addr);

    let token_info_1 = TokenInfo::CW20Token {
        contract_addr: token1_contract_addr.clone(),
    };
    
    let token_info_2 = TokenInfo::CW20Token {
        contract_addr: token2_contract_addr.clone(),
    };


    let factory_contract_addr = app.instantiate_contract(
        factory_code_id, 
        Addr::unchecked("Sender"),
        &FactoryInstantiate{
            pair_code_id,
        }, 
        &[],
        "Instantiate Factory",
        None,
    ).unwrap();

    println!("Factory contract addr: {}", factory_contract_addr);

    let router_contract_addr = app.instantiate_contract(
        router_code_id, 
        Addr::unchecked("Sender"), 
        &RouterInstantiate{
            factory_addr: factory_contract_addr.clone(),
        }, 
        &[], 
        "Instantiate Router", 
        None,
    ).unwrap();

    println!("Router contract addr: {}", router_contract_addr);

    //Test for Factory Contract
    let create_pair_res = app.execute_contract(
        Addr::unchecked("Sender"), 
        factory_contract_addr.clone(), 
        &FactoryExecuteMsg::CreateNewPair { 
            asset_infos: [token_info_1.clone(), token_info_2.clone()], 
        }, 
        &[],
    ).unwrap();

    println!("Create Pair Response: {:?} \n", create_pair_res);

    let query_res: PoolInfo = app.wrap().query_wasm_smart(
        factory_contract_addr, 
        &FactoryQueryMsg::Pair { 
            asset_infos: [token_info_1.clone(), token_info_2.clone()]
        },
    ).unwrap();

    println!("Query Pair Reponse: {:?} {:?}\n", query_res, query_res.pair_addr);

    let pair_contract_addr = query_res.pair_addr;

    // let allowance_before_res: AllowanceResponse = app.wrap().query_wasm_smart(
    //     Addr::unchecked(pair_contract_addr.clone()), 
    //     &PairQueryMsg::TokenQuery(
    //         cw20_base::msg::QueryMsg::Allowance { 
    //             owner: "user".to_string(), 
    //             spender: "user2".to_string() 
    //         }
    //     )
    // ).unwrap();

    // println!("Allowance before increasing: {:?}\n", allowance_before_res);

    // Call Pair contract with TokenExecuteMsg for IncreaseAllowance
    // let increase_allowance_res = app.execute_contract(
    //     Addr::unchecked("user"), 
    //     Addr::unchecked(pair_contract_addr.clone()),
    //     &PairExecuteMsg::TokenExecute(
    //         cw20::Cw20ExecuteMsg::IncreaseAllowance { 
    //             spender: "user2".to_string(), 
    //             amount: Uint128::from(100u128), 
    //             expires: None 
    //         } 
    //     ),
    //     &[]
    // ).unwrap();

    // println!("Increase Allowance on Pair {:?}\n", increase_allowance_res);

    // let allowance_after_res: AllowanceResponse = app.wrap().query_wasm_smart(
    //     Addr::unchecked(pair_contract_addr.clone()), 
    //     &PairQueryMsg::TokenQuery(
    //         cw20_base::msg::QueryMsg::Allowance { 
    //             owner: "user".to_string(), 
    //             spender: "user2".to_string() 
    //         }
    //     )
    // ).unwrap();

    // println!("Allowance after increasing: {:?}\n", allowance_after_res); 


    let token1_balance: BalanceResponse = app.wrap().query_wasm_smart(
        token1_contract_addr.clone(), 
        &Cw20QueryMsg::Balance { address: "user".to_string() }
        ,
    ).unwrap();

    let token2_balance: BalanceResponse = app.wrap().query_wasm_smart(
        token2_contract_addr.clone(), 
        &Cw20QueryMsg::Balance { address: "user".to_string() }
        ,
    ).unwrap();

    println!("Token Balances of the user: {:?}\n {:?}\n", token1_balance, token2_balance);


    let increase_allowance_token1 = app.execute_contract(
        Addr::unchecked("user"), 
        token1_contract_addr.clone(),
        &cw20::Cw20ExecuteMsg::IncreaseAllowance { 
            spender: pair_contract_addr.clone(), 
            amount: Uint128::from(200u128), 
            expires: None 
        },
        &[]
    ).unwrap();
    println!("Increased Allowance for Token1: {:?}\n", increase_allowance_token1);

    let increase_allowance_token2 = app.execute_contract(
        Addr::unchecked("user"), 
        token2_contract_addr.clone(),
        &cw20::Cw20ExecuteMsg::IncreaseAllowance { 
            spender: pair_contract_addr.clone(), 
            amount: Uint128::from(100u128), 
            expires: None 
        },
        &[]
    ).unwrap();

    println!("Increased Allowance for Token2: {:?}\n", increase_allowance_token2);

    // Test for Pair Contract
    let add_liquidity_res = app.execute_contract(
        Addr::unchecked("user"),
        Addr::unchecked(pair_contract_addr.clone()), 
        &PairExecuteMsg::AddLiquidity { 
            assets: [
                Token{
                    info: token_info_1.clone(),
                    amount: Uint128::from(100u128),
                },
                Token{
                    info: token_info_2.clone(),
                    amount: Uint128::from(100u128),
                }
            ], 
            min_liquidity_amt: Uint128::from(1u128)
        }, 
        &[],
    ).unwrap();

    println!("Add liquidity Response: {:?}\n", add_liquidity_res);

    let swap_res = app.execute_contract(
        Addr::unchecked("user"), 
        Addr::unchecked(pair_contract_addr.clone()), 
        &PairExecuteMsg::SwapAsset { 
            from_token: token_info_1.clone(), 
            to_token: token_info_2.clone(), 
            amount_in: 20u128, 
            min_amount_out: 5u128
        }, 
        &[],
    ).unwrap();

    println!("Swap Response: {:?}\n", swap_res);

    let withdraw_res = app.execute_contract(
        Addr::unchecked("user"), 
        Addr::unchecked(pair_contract_addr.clone()), 
        &PairExecuteMsg::RemoveLiquidity { 
            lp_token:Token {
                info: token_info_1.clone(),
                amount: Uint128::from(20u128),
            },
        }, 
        &[],
    );

    println!("Withdraw Res: {:?}\n", withdraw_res);
}

