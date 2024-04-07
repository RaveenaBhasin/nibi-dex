#!/usr/bin/env bash
getAddress() {
  #!/bin/bash
  # Store the output of 'nibid keys list' command
  output=$(nibid keys list <<<"$(source ./../../.env && echo $NIBI_KEY)")

  # Extract the addresses from the output using 'jq'
  addresses=($(echo "$output" | awk '/address:/ {print $3}'))

  echo "Please select an address:"

  # Prompt the user to select an address

  # Display the addresses with corresponding indices
  for index in "${!addresses[@]}"; do
    echo "$((index + 1)). ${addresses[index]}"
  done

  # Read user input for the selected index
  read -p "Enter the number corresponding to the selected address: " selected_index

  # Validate the user input
  if [[ ! "$selected_index" =~ ^[0-9]+$ ]] || ((selected_index < 1 || selected_index > ${#addresses[@]})); then
    echo "Invalid selection. Exiting."
    exit 1
  fi

  # Subtract 1 from the selected index to get the correct array index
  selected_index=$((selected_index - 1))

  # Get the selected address
  selected_address="${addresses[selected_index]}"

  # Save the selected address in a separate variable
  SAVED_ADDRESS="$selected_address"

  echo "Selected address: $selected_address"
  echo "Saved address: $SAVED_ADDRESS"
}

echo "Building for production...🟡"
cargo wasm
echo "✅ Build successful!........Enjoy 🚀"

# echo "Optimizing wasm..."
# cargo run-script optimize
# echo "✅ Optimized!........Enjoy 🚀"

# CHAIN_ID=nibiru-itn-1
CHAIN_ID=nibiru-testnet-1
echo "Now using chain id $CHAIN_ID...🟡"

nibid config broadcast-mode sync

# RPC="https://rpc.itn-1.nibiru.fi:443"
RPC="https://rpc.testnet-1.nibiru.fi:443"
NIBI=00unibi
# TXFLAG="--chain-id $CHAIN_ID --node $RPC --gas=1000000 --fees=250$NIBI --gas-adjustment 2"
TXFLAG="--chain-id $CHAIN_ID --node $RPC --gas=10000000 --fees=2500$NIBI --gas-adjustment 2"
echo "TXFLAG: $TXFLAG"

CARGO_TOML_PATH=./Cargo.toml
echo "CARGO_TOML_PATH: $CARGO_TOML_PATH"

PROJECT_NAME=$(awk -F ' *= *' '/\[package\]/{p=1} p&&/^name/{print $2; exit}' "$CARGO_TOML_PATH")

CLEANED_NAME=${PROJECT_NAME//\"/}
WASM_FILE="${CLEANED_NAME}.wasm"
echo "PROJECT_NAME: $PROJECT_NAME, WASM_FILE: $WASM_FILE"

getAddress

# RES=$(nibid tx wasm store ./../../artifacts/$WASM_FILE --from $SAVED_ADDRESS $TXFLAG -y --output json -b sync <<<"$(source ./../../.env && echo $NIBI_KEY)")
#
# echo "RES: $RES"
# echo "✅ wasm file uploaded on blockchain with $CHAIN_ID !........Enjoy 🚀"
#
# CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[-1].value')
#
# echo "✅ CODE_ID has been now generated: $CODE_ID"

# CODE_ID=249
# INIT='{
#     "pair_code_id": 250
# }'
# nibid tx wasm instantiate $CODE_ID "$INIT" --from $SAVED_ADDRESS --label "instantiate factory" --no-admin $TXFLAG -y <<< "$(source ./../../.env && echo $NIBI_KEY)"
#

# CODE_ID=251
# INIT='{
#

#
#

#

#
#     "factory_addr": "nibi1he7d3m60ytqe3uu3wxfzrxpagm7zwn27yjyfy4u620h5jdq4pqksc5aj7v"
# }'
# nibid tx wasm instantiate $CODE_ID "$INIT" --from $SAVED_ADDRESS --label "instantiate router" --no-admin $TXFLAG -y <<< "$(source ./../../.env && echo $NIBI_KEY)"

INIT='{
     "name" : "USDT",
     "symbol": "USDT",
     "decimals": 6,
     "initial_balances": [
         {
             "address": "nibi1e5lgey362kwkswas7khfvlqx9y70dhtkn7fq26",
             "amount": "1000000000"
         }
     ]
 }'
CODE_ID=349
nibid tx wasm instantiate $CODE_ID "$INIT" --from $SAVED_ADDRESS --label "instantiate mock coin" --no-admin $TXFLAG -y <<<"$(source ./../../.env && echo $NIBI_KEY)"

# CREATE_PAIR='{
#     "create_new_pair": {
#         "asset_infos": [
#             {
#                 "c_w20_token": {
#                     "contract_addr": "nibi1v3r5utz4uhpu74ua4y6nr57y8yvpcdsmgm238q3f5qp259lgkytsmej6x0"
#                 }
#             },
#             {
#                 "native_token" : {
#                     "denom" : "unibi"
#                 }
#             }
#         ]
#     }
# }'
# CONTRACT="nibi1he7d3m60ytqe3uu3wxfzrxpagm7zwn27yjyfy4u620h5jdq4pqksc5aj7v"
# nibid tx wasm execute $CONTRACT "$CREATE_PAIR" --from $SAVED_ADDRESS $TXFLAG -y <<< "$(source ./../../.env && echo $NIBI_KEY)"

# QUERY_PAIR='{
#     "pair": {
#         "asset_infos":[
#             {
#                 "c_w20_token": {
#                     "contract_addr":"nibi1v3r5utz4uhpu74ua4y6nr57y8yvpcdsmgm238q3f5qp259lgkytsmej6x0"
#                 }
#             },
#             {
#                 "native_token": {
#                     "denom":"unibi"
#                 }
#             }
#         ]
#     }
# }'
# //pair
# CONTRACT="nibi16j3057pchfy0fjwg749kkcve3cyusv2pg78stat6snncwayksu3qq39d0q"

# // factory
# CONTRACT="nibi1he7d3m60ytqe3uu3wxfzrxpagm7zwn27yjyfy4u620h5jdq4pqksc5aj7v"
# nibid query wasm contract-state smart $CONTRACT "$QUERY_PAIR" --node $RPC --output json

# INCREASE_ALLOWANCE='{
#     "increase_allowance": {
#         "spender": "nibi1jfuzdxd92h66z5g0k4ewgyl5lfzj7d9k089aqfn4tk68hd83ne2q2v8ump",
#         "amount": "1000"
#     }
# }'

# CONTRACT="nibi1v3r5utz4uhpu74ua4y6nr57y8yvpcdsmgm238q3f5qp259lgkytsmej6x0"
# nibid tx wasm execute $CONTRACT "$INCREASE_ALLOWANCE" --from $SAVED_ADDRESS $TXFLAG -y <<< "$(source ./../../.env && echo $NIBI_KEY)"

# ADD_LIQ='{
#     "add_liquidity": {
#         "assets": [
#             {
#                 "info": {
#                     "c_w20_token": {
#                         "contract_addr": "nibi1v3r5utz4uhpu74ua4y6nr57y8yvpcdsmgm238q3f5qp259lgkytsmej6x0"
#                     }
#                 },
#                 "amount":"1000"
#             },
#             {
#                 "info": {
#                     "native_token" : {
#                         "denom" : "unibi"
#                     }
#                 },
#                 "amount": "1000"
#             }
#         ],
#         "min_liquidity_amt": "0"
#     }
# }'
# CONTRACT="nibi1jfuzdxd92h66z5g0k4ewgyl5lfzj7d9k089aqfn4tk68hd83ne2q2v8ump"
# nibid tx wasm execute $CONTRACT "$ADD_LIQ" --from $SAVED_ADDRESS $TXFLAG -y <<< "$(source ./../../.env && echo $NIBI_KEY)"

# SWAP='{
#     "swap_asset": {
#         "from_token": {
#             "c_w20_token": {
#                 "contract_addr":"nibi1v3r5utz4uhpu74ua4y6nr57y8yvpcdsmgm238q3f5qp259lgkytsmej6x0"
#             }
#         },
#         "to_token": {
#             "native_token": {
#                 "denom": "unibi"
#             }
#         },
#         "amount_in": "10",
#         "min_amount_out": "1"
#     }
# }'
# CONTRACT="nibi1jfuzdxd92h66z5g0k4ewgyl5lfzj7d9k089aqfn4tk68hd83ne2q2v8ump"
# nibid tx wasm execute $CONTRACT "$SWAP" --from $SAVED_ADDRESS $TXFLAG -y <<<"$(source ./../../.env && echo $NIBI_KEY)"

# QUERY_ALLOWANCE='{
#     "allowance": {
#         "owner": "nibi1e5lgey362kwkswas7khfvlqx9y70dhtkn7fq26",
#         "spender": "nibi1jfuzdxd92h66z5g0k4ewgyl5lfzj7d9k089aqfn4tk68hd83ne2q2v8ump"
#     }
# }'
# CONTRACT="nibi1v3r5utz4uhpu74ua4y6nr57y8yvpcdsmgm238q3f5qp259lgkytsmej6x0"
# nibid query wasm contract-state smart $CONTRACT "$QUERY_ALLOWANCE" --node $RPC --output json

# TOKEN_QUERY='{
#     "token_query": {
#             "balance": {
#                 "address": "nibi1e5lgey362kwkswas7khfvlqx9y70dhtkn7fq26"
#             }
#     }

# }'
# CONTRACT="nibi1jfuzdxd92h66z5g0k4ewgyl5lfzj7d9k089aqfn4tk68hd83ne2q2v8ump"
# nibid query wasm contract-state smart $CONTRACT "$TOKEN_QUERY" --node $RPC --output json
