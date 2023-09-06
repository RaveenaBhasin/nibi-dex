use cosmwasm_std::{Empty, Addr, Uint128};
use cw20::Cw20Coin;
use cw_multi_test::{App, Contract, ContractWrapper, Executor};
use cw20_base::msg::InstantiateMsg as Cw20InstantiateMsg;
use packages::pair::{TokenInfo, InstantiateMsg as PairInstantiate};
use packages::factory::{InstantiateMsg as FactoryInstantiate, ExecuteMsg as FactoryExecuteMsg};
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
    );
    // .with_reply(factory::contract::reply);
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

#[test]
fn integration_test() {

    let mut router = mock_app();

    //Store code

    let pair_code_id = router.store_code(pair_contract());
    println!("Pair code id: {}", pair_code_id);

    let factory_code_id = router.store_code(factory_contract());
    println!("Factory code id: {}", factory_code_id);

    let router_code_id = router.store_code(router_contract());
    println!("Router code id: {}", router_code_id);

    let token_info_1 = TokenInfo::CW20Token {
        contract_addr: Addr::unchecked("token1_contract_address"),
    };
    
    let token_info_2 = TokenInfo::CW20Token {
        contract_addr: Addr::unchecked("token2_contract_address"),
    };

    //Instantiate Contract

    let pair_contract_addr = router.instantiate_contract(
        pair_code_id, 
        Addr::unchecked("Sender"), 
        &PairInstantiate {
            token_info: [token_info_1, token_info_2],
            lp_token_decimal: 8u8,
            cw20_instantiate: Cw20InstantiateMsg {
                name: "CW20 Token".to_string(),      
                symbol: "CWT".to_string(),            
                decimals: 18u8,                        
                initial_balances: vec![                
                    Cw20Coin {
                        address: "token_holder_address_1".to_string(),
                        amount: Uint128::from(1000000u128),
                    },
                    Cw20Coin {
                        address: "token_holder_address_2".to_string(),
                        amount: Uint128::from(500000u128),
                    },
                ],
                mint: None,
                marketing: None,
            },
        }, 
        &[], 
        "Instantiate Factory ", 
        None,
    ).unwrap();

    println!("Pair contract addr: {}", pair_contract_addr);

    let factory_contract_addr = router.instantiate_contract(
        factory_code_id, 
        Addr::unchecked("Sender"),
        &FactoryInstantiate{
            pair_code_id: pair_code_id,
        }, 
        &[],
        "Instantiate Factory",
        None,
    ).unwrap();

    println!("Factory contract addr: {}", factory_contract_addr);

    let router_contract_addr = router.instantiate_contract(
        router_code_id, 
        Addr::unchecked("Sender"), 
        &RouterInstantiate{
            factory_addr: factory_contract_addr,
        }, 
        &[], 
        "Instantiate Router", 
        None,
    ).unwrap();

    println!("Router contract addr: {}", router_contract_addr);

}

