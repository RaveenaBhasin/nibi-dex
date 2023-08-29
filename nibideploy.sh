getAddress() {
    #!/bin/bash
    # Store the output of 'junod keys list' command
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

CHAIN_ID=nibiru-itn-1
echo "Now using chain id $CHAIN_ID...🟡"

nibid config broadcast-mode sync

RPC="https://rpc.itn-1.nibiru.fi:443"
NIBI=00unibi
TXFLAG="--chain-id $CHAIN_ID --node $RPC --gas=130000 --fees=35$NIBI --gas-adjustment 1.3"
echo "TXFLAG: $TXFLAG"

CARGO_TOML_PATH=./Cargo.toml
echo "CARGO_TOML_PATH: $CARGO_TOML_PATH"

PROJECT_NAME=$(awk -F ' *= *' '/\[package\]/{p=1} p&&/^name/{print $2; exit}' "$CARGO_TOML_PATH")

CLEANED_NAME=${PROJECT_NAME//\"/}
WASM_FILE="${CLEANED_NAME}.wasm"
echo "PROJECT_NAME: $PROJECT_NAME, WASM_FILE: $WASM_FILE"

getAddress


# RES=$(nibid tx wasm store ./../../artifacts/$WASM_FILE --from $SAVED_ADDRESS $TXFLAG -y --output json -b sync <<< "$(source ./../../.env && echo $NIBI_KEY)")

# echo "RES: $RES"
# echo "✅ wasm file uploaded on blockchain with $CHAIN_ID !........Enjoy 🚀"

# CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[-1].value')

# echo "✅ CODE_ID has been now generated: $CODE_ID"

# INIT='{
#     "factory_addr": "nibi1hk4kcllz9lsd5haldt3pkzmwer7psfrn6lj4kpular6d3uh0yr4sevftld"
# }'
# CODE_ID=31546
# INIT='{
#     "pair_code_id": 31564
# }'
#CODE_ID=31505
# nibid tx wasm instantiate $CODE_ID "$INIT" --from $SAVED_ADDRESS --label "instantiate router" --no-admin $TXFLAG -y <<< "$(source ./../../.env && echo $NIBI_KEY)"

# INIT='{
#     "name" : "nibiDexMockToken", 
#     "symbol": "NMT", 
#     "decimals": 6,
#     "initial_balances": [
#         {
#             "address": "nibi17cu3tjp5alp2c3umzhamygeyj2rkv4kshq6l9n", 
#             "amount": "1000000000"
#         }
#     ]
# }'
# CODE_ID=31555
# nibid tx wasm instantiate $CODE_ID "$INIT" --from $SAVED_ADDRESS --label "instantiate mock coin" --no-admin $TXFLAG -y <<< "$(source ./../../.env && echo $NIBI_KEY)"


# CREATE_PAIR='{
#     "create_new_pair": {
#         "asset_infos": [
#             {
#                 "c_w20_token": {
#                     "contract_addr": "nibi1fs882tudg4pegayj5fpess4mkupa77tx9x4chndwpezqkjegzhfsqlnkew"
#                 }
#             },
#             {
#                 "c_w20_token": {
#                     "contract_addr": "nibi1xddyc03asynlf74tkfuhqs0t4qtljv7md5rawd6dk7nm2s9teckqphgjfn"
#                 }
#             }
#         ]
#     }    
# }'
# CONTRACT="nibi1hk4kcllz9lsd5haldt3pkzmwer7psfrn6lj4kpular6d3uh0yr4sevftld"
# nibid tx wasm execute $CONTRACT "$CREATE_PAIR" --from $SAVED_ADDRESS $TXFLAG -y <<< "$(source ./../../.env && echo $NIBI_KEY)"

# QUERY_PAIR='{
#     "pair": {
#         "asset_infos":[
#             {
#                 "c_w20_token": {
#                     "contract_addr":"nibi1fs882tudg4pegayj5fpess4mkupa77tx9x4chndwpezqkjegzhfsqlnkew"
#                 }
#             },
#             {   
#                 "c_w20_token": {
#                     "contract_addr":"nibi1xddyc03asynlf74tkfuhqs0t4qtljv7md5rawd6dk7nm2s9teckqphgjfn"
#                 }
#             }
#         ]
#     }
# }'
# //pair
# CONTRACT="nibi16j3057pchfy0fjwg749kkcve3cyusv2pg78stat6snncwayksu3qq39d0q" 

# // factory
# CONTRACT="nibi1hk4kcllz9lsd5haldt3pkzmwer7psfrn6lj4kpular6d3uh0yr4sevftld" 
# nibid query wasm contract-state smart $CONTRACT "$QUERY_PAIR" --node $RPC --output json

# SWAP='{
#     "swap_asset": {
#         "from_token": {
#             "c_w20_token": {
#                 "contract_addr":"nibi1fs882tudg4pegayj5fpess4mkupa77tx9x4chndwpezqkjegzhfsqlnkew"
#             }
#         },
#         "to_token": {   
#             "c_w20_token": {
#                 "contract_addr":"nibi1xddyc03asynlf74tkfuhqs0t4qtljv7md5rawd6dk7nm2s9teckqphgjfn"
#             }
#         }, 
#         "amount_in": "10000000",
#         "min_amount_out": "1000000"
#     }
# }'
# CONTRACT="nibi1lgw5dhy3ee7p2g0e00ljhwjxxmg9pqkdh2z27yr738u7lpdsmhps99x4hk"
# nibid tx wasm execute $CONTRACT "$SWAP" --from $SAVED_ADDRESS $TXFLAG -y <<< "$(source ./../../.env && echo $NIBI_KEY)"


ADD_LIQ='{
    "add_liquidity": {
        "assets": [
            {
                "info": {
                    "c_w20_token": {
                        "contract_addr": "nibi1fs882tudg4pegayj5fpess4mkupa77tx9x4chndwpezqkjegzhfsqlnkew"
                    }
                },
                "amount":"10000000"
            },
            {
                "info": {
                    "c_w20_token": {
                        "contract_addr": "nibi1xddyc03asynlf74tkfuhqs0t4qtljv7md5rawd6dk7nm2s9teckqphgjfn" 
                    }
                },
                "amount": "10000000"
            }
        ],
        "min_liquidity_amt": "1000000"
    }
}'
CONTRACT="nibi16j3057pchfy0fjwg749kkcve3cyusv2pg78stat6snncwayksu3qq39d0q"
nibid tx wasm execute $CONTRACT "$ADD_LIQ" --from $SAVED_ADDRESS $TXFLAG -y <<< "$(source ./../../.env && echo $NIBI_KEY)"
