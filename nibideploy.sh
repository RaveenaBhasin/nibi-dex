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
NIBI=000000unibi
TXFLAG="--chain-id $CHAIN_ID --node $RPC --gas=3000000 --fees=2$NIBI --gas-adjustment 1.3"
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

INIT='{"pair_code_id": 31503}'
CODE_ID=31505
nibid tx wasm instantiate $CODE_ID "$INIT" --from $SAVED_ADDRESS --label "instantiate factory" --no-admin $TXFLAG -y <<< "$(source ./../../.env && echo $NIBI_KEY)"