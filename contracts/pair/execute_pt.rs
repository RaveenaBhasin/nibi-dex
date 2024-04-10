use crate::state::PAIR_INFO;
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    to_binary, Decimal256, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult, Uint128,
};
use cw20::Cw20ExecuteMsg;
use packages::pair::PairInfo;

// version info for migration info
const _CONTRACT_NAME: &str = "crates.io:nibiru-hack";
const _CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
const FEE_SCALING_FACTOR: Uint128 = Uint128::new(10_000);
use crate::query_pt::query;

pub mod execute {

    use cosmwasm_std::{Addr, BankMsg, Coin, CosmosMsg, Empty, Uint128, WasmMsg};
    use cw20_base::{
        state::{BALANCES, TOKEN_INFO},
        ContractError,
    };

    use crate::state::FEES;

    use super::*;
    use packages::pair::{Token, TokenInfo};

    pub fn swap(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        from_token: TokenInfo,
        to_token: TokenInfo,
        amount_in: u128,
        min_amount_out: u128,
    ) -> StdResult<Response> {
        let pair_info: PairInfo = PAIR_INFO.load(deps.storage).unwrap();

        if from_token == to_token {
            return Err(StdError::generic_err("Cannot swap same token"));
        }

        if (from_token != pair_info.assets[0] && from_token != pair_info.assets[1])
            || (to_token != pair_info.assets[0] && to_token != pair_info.assets[1])
        {
            return Err(StdError::generic_err("Pair does not exist"));
        }

        // Funds sent should not just be equal, but greater than equal
        // funds array should just be checked in case of native tokens only
        // Funds should not be asserted as equality but include_str!("")

        // if info.funds != vec![Coin {
        //     amount: Uint128::from(tok)
        // }])

        // let expected_sent_fund = Coin{
        //     denom:
        // }

        let mut res = Response::new();
        match &from_token {
            TokenInfo::CW20Token { contract_addr } => {
                let asset_transfer = CosmosMsg::Wasm(WasmMsg::Execute {
                    contract_addr: contract_addr.to_string(),
                    msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
                        owner: info.sender.to_string(),
                        recipient: env.contract.address.to_string().clone(),
                        amount: amount_in.into(),
                    })?,
                    funds: vec![],
                });
                res = res.add_message(asset_transfer);
            }
            TokenInfo::NativeToken { denom: _denom } => {
                let sent_fund = info
                    .funds
                    .get(0)
                    .ok_or_else(|| StdError::generic_err("No funds sent"))
                    .unwrap();
                println!("Sent fund {:?}", sent_fund);
                if sent_fund.clone().amount.u128() < amount_in {
                    return Err(StdError::generic_err("Insufficient funds sent"));
                };
                if sent_fund.denom != "unibi".to_string() {
                    return Err(StdError::generic_err("Invalid denomination"));
                };
            }
        };

        let amount_out = calculate_swap_amount(
            deps.as_ref(),
            env,
            info.clone(),
            from_token.clone(),
            to_token.clone(),
            amount_in,
        )?;
        println!("Amount out {:?}", amount_out);

        let mut msgs: Vec<CosmosMsg> = vec![];

        let fees = FEES.load(deps.storage)?;
        let protocol_fees_amount =
            get_protocol_fees(Uint128::from(amount_in), fees.protocol_fee_percent);
        if !protocol_fees_amount.is_zero() {
            msgs.push(get_fee_transfer_msg(
                &info.sender,
                &fees.protocol_fee_recipient,
                from_token.clone(),
                protocol_fees_amount,
            )?)
        }

        if amount_out < min_amount_out {
            return Err(StdError::generic_err("Insufficient amount out"));
        }

        // let res = Response::new().add_message(token_transfer);
        let token_transfer: CosmosMsg<Empty> = match &to_token {
            TokenInfo::CW20Token { contract_addr } => CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr: contract_addr.to_string(),
                msg: to_binary(&Cw20ExecuteMsg::Transfer {
                    recipient: info.sender.to_string().clone(),
                    amount: amount_out.into(),
                })?,
                funds: vec![],
            }),
            TokenInfo::NativeToken { denom } => CosmosMsg::Bank(BankMsg::Send {
                to_address: info.sender.to_string().clone(),
                amount: vec![Coin {
                    denom: denom.to_string().clone(),
                    amount: amount_out.into(),
                }],
            }),
        };
        msgs.push(token_transfer);

        Ok(res.add_messages(msgs))
    }

    fn calculate_swap_amount(
        deps: Deps,
        env: Env,
        _info: MessageInfo,
        from_token: TokenInfo,
        to_token: TokenInfo,
        amount_in: u128,
    ) -> StdResult<u128> {
        let mut token_balances = vec![];
        let this_address = env.contract.address.clone();
        let assets = [from_token.clone(), to_token.clone()];
        for (_, asset) in assets.iter().enumerate() {
            let token_bal = match &asset {
                TokenInfo::CW20Token { contract_addr } => query::query_token_balance(
                    &deps.querier,
                    contract_addr.clone(),
                    this_address.clone(),
                )?,
                TokenInfo::NativeToken { denom } => query::query_native_balance(
                    &deps.querier,
                    this_address.clone(),
                    denom.to_string(),
                )?,
            };
            token_balances.push(token_bal);
        }
        let amount_in = Uint128::from(amount_in);

        // x * y = k = (x+a) * (y-b)
        // (x * y) / (x+a) = (y-b)
        // (x * y) / (x+a) + b = y
        // b = y - ( (x * y) / (x+a) )
        // let amount_out = token_balances[1].u128()
        //     - (token_balances[1] * token_balances[0]).u128()
        //         / (token_balances[0].u128() + amount_in);

        // let protocol_fees_amount= (amount_in
        //     .full_mul(fees.protocol_fee_percent)
        //     .checked_div(Uint256::from(FEE_SCALING_FACTOR)))?;
        // let lp_fees_amount: Uint128 = (Uint128::from(amount_in)
        //     .full_mul(fees.lp_fee_percent)
        //     .checked_div(Uint256::from(FEE_SCALING_FACTOR)))
        // .into()?;
        // let protocol_fees_amount = amount_in * (fees.protocol_fee_percent) / (FEE_SCALING_FACTOR);
        let fees = FEES.load(deps.storage)?;
        let protocol_fees_amount = get_protocol_fees(amount_in, fees.protocol_fee_percent);
        let lp_fees_amount = amount_in * (fees.lp_fee_percent) / (FEE_SCALING_FACTOR);

        let amount_out = (token_balances[1] * (amount_in - protocol_fees_amount - lp_fees_amount))
            / (token_balances[0] + (amount_in - protocol_fees_amount - lp_fees_amount));
        println!(
            "Logging inside contract swap function{:?} {:?} {:?} ",
            assets, amount_out, token_balances
        );
        Ok(amount_out.u128())
    }

    pub fn get_protocol_fees(amount_in: Uint128, fee_percent: Uint128) -> Uint128 {
        amount_in * (fee_percent) / (FEE_SCALING_FACTOR)
    }

    pub fn execute_mint(
        deps: DepsMut,
        _env: Env,
        _info: MessageInfo,
        recipient: String,
        amount: Uint128,
    ) -> Result<Response, ContractError> {
        let mut config = TOKEN_INFO.load(deps.storage)?;
        // update supply and enforce cap
        config.total_supply += amount;
        if let Some(limit) = config.get_cap() {
            if config.total_supply > limit {
                return Err(ContractError::CannotExceedCap {});
            }
        }
        TOKEN_INFO.save(deps.storage, &config)?;

        // add amount to recipient balance
        let rcpt_addr = deps.api.addr_validate(&recipient)?;
        BALANCES.update(
            deps.storage,
            &rcpt_addr,
            |balance: Option<Uint128>| -> StdResult<_> { Ok(balance.unwrap_or_default() + amount) },
        )?;

        let res = Response::new()
            .add_attribute("action", "mint")
            .add_attribute("to", recipient)
            .add_attribute("amount", amount);
        Ok(res)
    }

    fn get_cw20_transfer_msg(
        owner: &Addr,
        recipient: &Addr,
        token_addr: &Addr,
        token_amount: Uint128,
    ) -> StdResult<CosmosMsg> {
        let transfer_cw20_msg = Cw20ExecuteMsg::TransferFrom {
            owner: owner.into(),
            recipient: recipient.into(),
            amount: token_amount,
        };
        let exec_cw20_transfer = WasmMsg::Execute {
            contract_addr: token_addr.into(),
            msg: to_binary(&transfer_cw20_msg)?,
            funds: vec![],
        };
        let cw20_transfer_cosmos_msg: CosmosMsg = exec_cw20_transfer.into();
        Ok(cw20_transfer_cosmos_msg)
    }

    fn get_bank_transfer_msg(
        recipient: &Addr,
        denom: &str,
        native_amount: Uint128,
    ) -> StdResult<CosmosMsg> {
        let transfer_bank_msg = cosmwasm_std::BankMsg::Send {
            to_address: recipient.into(),
            amount: vec![Coin {
                denom: denom.to_string(),
                amount: native_amount,
            }],
        };

        let transfer_bank_cosmos_msg: CosmosMsg = transfer_bank_msg.into();
        Ok(transfer_bank_cosmos_msg)
    }

    fn get_fee_transfer_msg(
        sender: &Addr,
        recipient: &Addr,
        from_token: TokenInfo,
        amount: Uint128,
    ) -> StdResult<CosmosMsg> {
        match from_token {
            TokenInfo::CW20Token { contract_addr } => {
                get_cw20_transfer_msg(sender, recipient, &contract_addr, amount)
            }
            TokenInfo::NativeToken { denom } => get_bank_transfer_msg(sender, &denom, amount),
        }
    }

    pub fn add_liquidity(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        assets: [Token; 2],
        min_liquidity: Uint128,
    ) -> StdResult<Response> {
        // check if the pair exists
        let pair_info: PairInfo = PAIR_INFO.load(deps.storage).unwrap();
        if !((assets[0].info == pair_info.assets[0] && assets[1].info == pair_info.assets[1])
            || (assets[0].info == pair_info.assets[1] && assets[1].info == pair_info.assets[0]))
        {
            return Err(StdError::generic_err("Pair does not exist"));
        }

        println!("Transferring the tokens !!");
        // transfer from both the asset amounts
        let mut messages = vec![];
        for (_i, asset) in assets.iter().enumerate() {
            let _asset_transfer = match &asset.info {
                TokenInfo::CW20Token { contract_addr } => {
                    let asset_transfer: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Execute {
                        contract_addr: contract_addr.to_string(),
                        msg: to_binary(&Cw20ExecuteMsg::TransferFrom {
                            owner: info.sender.to_string(),
                            recipient: env.contract.address.to_string().clone(),
                            amount: asset.amount,
                        })?,
                        funds: vec![],
                    });
                    messages.push(asset_transfer);
                }
                TokenInfo::NativeToken { denom: _denom } => {}
            };
        }

        println!("All tokens transferred successfully !!");

        let mut token_balances = vec![];
        for (_, asset) in assets.iter().enumerate() {
            let token_bal = match &asset.info {
                TokenInfo::CW20Token { contract_addr } => query::query_token_balance(
                    &deps.querier,
                    contract_addr.clone(),
                    env.contract.address.clone(),
                )?,
                TokenInfo::NativeToken { denom } => query::query_native_balance(
                    &deps.querier,
                    env.contract.address.clone(),
                    denom.to_string(),
                )?,
            };
            //            if token_bal == Uint128::from(0u128) {
            //                return Err(StdError::generic_err(format!(
            //                    "Balance found zero {:?}",
            //                    asset.info
            //                )));
            //            }
            token_balances.push(token_bal);
        }

        let asset0_value = assets[0].amount;
        let asset1_value = assets[1].amount;

        let token_info = TOKEN_INFO.load(deps.storage)?;
        let res = cw20::TokenInfoResponse {
            name: token_info.name,
            symbol: token_info.symbol,
            decimals: token_info.decimals,
            total_supply: token_info.total_supply,
        };

        let liquidity_minted: Uint128;
        println!("Token balances {:?}", token_balances);
        if res.total_supply == Uint128::from(0u128) {
            liquidity_minted = std::cmp::min(asset0_value, asset1_value);
        } else {
            liquidity_minted = std::cmp::min(
                asset0_value.multiply_ratio(res.total_supply, token_balances[0]),
                asset1_value.multiply_ratio(res.total_supply, token_balances[1]),
            )
        }

        if liquidity_minted < min_liquidity {
            return Err(StdError::generic_err(format!(
                "Insufficient liquidity minted {:?}",
                liquidity_minted
            )));
        }

        // mint the lp token, to the sender
        execute_mint(
            deps,
            env,
            info.clone(),
            info.sender.to_string().clone(),
            liquidity_minted,
        )
        .unwrap();

        // store the lp token balance in the state
        return Ok(Response::new().add_messages(messages).add_attributes(vec![
            ("action", "add_liquidity"),
            ("sender", info.sender.as_str()),
            ("amount", liquidity_minted.to_string().as_str()),
        ]));
    }

    pub fn withdraw_liquidity(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        lp_token: Token,
    ) -> StdResult<Response> {
        let this_address = env.contract.address.clone();

        let pair_info: PairInfo = PAIR_INFO.load(deps.storage).unwrap();
        let token_info = TOKEN_INFO.load(deps.storage)?;
        let res = cw20::TokenInfoResponse {
            name: token_info.name,
            symbol: token_info.symbol,
            decimals: token_info.decimals,
            total_supply: token_info.total_supply,
        };

        let _ratio = Decimal256::from_ratio(lp_token.amount, res.total_supply);

        let mut token_balances = vec![];
        for (_, asset) in pair_info.assets.iter().enumerate() {
            let token_bal = match &asset {
                TokenInfo::CW20Token { contract_addr } => query::query_token_balance(
                    &deps.querier,
                    contract_addr.clone(),
                    this_address.clone(),
                )?,
                TokenInfo::NativeToken { denom } => query::query_native_balance(
                    &deps.querier,
                    this_address.clone(),
                    denom.to_string(),
                )?,
            };
            token_balances.push(token_bal);
        }

        let assets_returned = [
            lp_token
                .amount
                .multiply_ratio(token_balances[0], res.total_supply),
            lp_token
                .amount
                .multiply_ratio(token_balances[1], res.total_supply),
        ];

        cw20_base::contract::execute_burn(deps, env, info.clone(), lp_token.amount).unwrap();

        let mut messages = vec![];
        for (i, asset) in pair_info.assets.iter().enumerate() {
            match &asset {
                TokenInfo::CW20Token { contract_addr } => {
                    let cw20_asset_transfer: CosmosMsg<Empty> = CosmosMsg::Wasm(WasmMsg::Execute {
                        contract_addr: contract_addr.to_string(),
                        msg: to_binary(&Cw20ExecuteMsg::Transfer {
                            recipient: info.sender.clone().to_string(),
                            amount: assets_returned[i],
                        })?,
                        funds: vec![],
                    });
                    messages.push(cw20_asset_transfer);
                }
                TokenInfo::NativeToken { denom } => {
                    let native_asset_transfer: CosmosMsg<Empty> = CosmosMsg::Bank(BankMsg::Send {
                        to_address: info.sender.to_string().clone(),
                        amount: vec![Coin {
                            denom: denom.to_string().clone(),
                            amount: assets_returned[i],
                        }],
                    });
                    messages.push(native_asset_transfer);
                }
            };
        }

        Ok(Response::new().add_messages(messages).add_attributes(vec![
            ("action", "withdraw liquidity"),
            (
                "Assets returned",
                &format!("{}, {}", assets_returned[0], assets_returned[1]),
            ),
        ]))
    }
}
