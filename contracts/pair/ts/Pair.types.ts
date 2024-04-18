/**
* This file was automatically generated by @cosmwasm/ts-codegen@0.35.7.
* DO NOT MODIFY IT BY HAND. Instead, modify the source JSONSchema file,
* and run the @cosmwasm/ts-codegen generate command to regenerate this file.
*/

export type Uint128 = string;
export type Logo = {
  url: string;
} | {
  embedded: EmbeddedLogo;
};
export type EmbeddedLogo = {
  svg: Binary;
} | {
  png: Binary;
};
export type Binary = string;
export type TokenInfo = {
  c_w20_token: {
    contract_addr: Addr;
  };
} | {
  native_token: {
    denom: string;
  };
};
export type Addr = string;
export interface InstantiateMsg {
  cw20_instantiate: InstantiateMsg1;
  lp_token_decimal: number;
  token_info: [TokenInfo, TokenInfo];
  treasury: Addr;
}
export interface InstantiateMsg1 {
  decimals: number;
  initial_balances: Cw20Coin[];
  marketing?: InstantiateMarketingInfo | null;
  mint?: MinterResponse | null;
  name: string;
  symbol: string;
}
export interface Cw20Coin {
  address: string;
  amount: Uint128;
}
export interface InstantiateMarketingInfo {
  description?: string | null;
  logo?: Logo | null;
  marketing?: string | null;
  project?: string | null;
}
export interface MinterResponse {
  cap?: Uint128 | null;
  minter: string;
}
export type ExecuteMsg = {
  swap_asset: {
    amount_in: number;
    from_token: TokenInfo;
    min_amount_out: number;
    to_token: TokenInfo;
  };
} | {
  add_liquidity: {
    assets: [Token, Token];
    min_liquidity_amt: Uint128;
  };
} | {
  remove_liquidity: {
    lp_token_amount: Uint128;
  };
} | {
  token_execute: Cw20ExecuteMsg;
};
export type Cw20ExecuteMsg = {
  transfer: {
    amount: Uint128;
    recipient: string;
  };
} | {
  burn: {
    amount: Uint128;
  };
} | {
  send: {
    amount: Uint128;
    contract: string;
    msg: Binary;
  };
} | {
  increase_allowance: {
    amount: Uint128;
    expires?: Expiration | null;
    spender: string;
  };
} | {
  decrease_allowance: {
    amount: Uint128;
    expires?: Expiration | null;
    spender: string;
  };
} | {
  transfer_from: {
    amount: Uint128;
    owner: string;
    recipient: string;
  };
} | {
  send_from: {
    amount: Uint128;
    contract: string;
    msg: Binary;
    owner: string;
  };
} | {
  burn_from: {
    amount: Uint128;
    owner: string;
  };
} | {
  mint: {
    amount: Uint128;
    recipient: string;
  };
} | {
  update_minter: {
    new_minter?: string | null;
  };
} | {
  update_marketing: {
    description?: string | null;
    marketing?: string | null;
    project?: string | null;
  };
} | {
  upload_logo: Logo;
};
export type Expiration = {
  at_height: number;
} | {
  at_time: Timestamp;
} | {
  never: {};
};
export type Timestamp = Uint64;
export type Uint64 = string;
export interface Token {
  amount: Uint128;
  info: TokenInfo;
}
export type QueryMsg = {
  balance: {
    address: string;
  };
} | {
  token_info: {};
} | {
  minter: {};
} | {
  allowance: {
    owner: string;
    spender: string;
  };
} | {
  all_allowances: {
    limit?: number | null;
    owner: string;
    start_after?: string | null;
  };
} | {
  all_spender_allowances: {
    limit?: number | null;
    spender: string;
    start_after?: string | null;
  };
} | {
  all_accounts: {
    limit?: number | null;
    start_after?: string | null;
  };
} | {
  marketing_info: {};
} | {
  download_logo: {};
};
export type QueryMsg1 = {
  balance: {
    address: string;
  };
} | {
  token_info: {};
} | {
  minter: {};
} | {
  allowance: {
    owner: string;
    spender: string;
  };
} | {
  all_allowances: {
    limit?: number | null;
    owner: string;
    start_after?: string | null;
  };
} | {
  all_spender_allowances: {
    limit?: number | null;
    spender: string;
    start_after?: string | null;
  };
} | {
  all_accounts: {
    limit?: number | null;
    start_after?: string | null;
  };
} | {
  marketing_info: {};
} | {
  download_logo: {};
};
export type ArraySize_2OfToken = [Token, Token];
export interface PairInfo {
  assets: [TokenInfo, TokenInfo];
  lp_token_decimal: number;
}