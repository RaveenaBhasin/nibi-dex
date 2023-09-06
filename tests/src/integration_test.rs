use cosmwasm_std::{Empty, Addr, Uint128};
use cw20::Cw20Coin;
use cw_multi_test::{App, Contract, ContractWrapper, Executor};
use cw20_base::msg::InstantiateMsg as Cw20InstantiateMsg;
use factory::state::PoolInfo;
use packages::pair::{TokenInfo, InstantiateMsg as PairInstantiateMsg, ExecuteMsg as PairExecuteMsg, Token};
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

    // let pair_contract_addr = app.instantiate_contract(
    //     pair_code_id, 
    //     Addr::unchecked("Sender"), 
    //     &PairInstantiateMsg {
    //         token_info: [token_info_1.clone(), token_info_2.clone()],
    //         lp_token_decimal: 8u8,
    //         cw20_instantiate: Cw20InstantiateMsg {
    //             name: "CW20 Token".to_string(),      
    //             symbol: "CWT".to_string(),            
    //             decimals: 18u8,                        
    //             initial_balances: vec![                
    //                 Cw20Coin {
    //                     address: token1_contract_addr.to_string(),
    //                     amount: Uint128::from(1000000u128),
    //                 },
    //                 Cw20Coin {
    //                     address: token2_contract_addr.to_string(),
    //                     amount: Uint128::from(500000u128),
    //                 },
    //             ],
    //             mint: None,
    //             marketing: None,
    //         },
    //     }, 
    //     &[], 
    //     "Instantiate Factory ", 
    //     None,
    // ).unwrap();

    // println!("Pair contract addr: {}", pair_contract_addr);

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

    println!("Query Pair Reponse: {:?}", query_res);

    //Test for Pair Contract
    // let add_liquidity_res = app.execute_contract(
    //     Addr::unchecked("user"),
    //     pair_contract_addr.clone(), 
    //     &PairExecuteMsg::AddLiquidity { 
    //         assets: [
    //             Token{
    //                 info: token_info_1.clone(),
    //                 amount: Uint128::from(100u128),
    //             },
    //             Token{
    //                 info: token_info_2.clone(),
    //                 amount: Uint128::from(100u128),
    //             }
    //         ], 
    //         min_liquidity_amt: Uint128::from(123u128)
    //     }, 
    //     &[],
    // ).unwrap();

    // println!("Add liquidity Response: {:?}", add_liquidity_res);


}

