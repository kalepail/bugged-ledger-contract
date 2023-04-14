set -e

cargo build --target wasm32-unknown-unknown --release

contract_id=$(soroban contract deploy --wasm target/wasm32-unknown-unknown/release/bugged_ledger_contract.wasm --source SBCKNWG5O3SRVA26LPWGBMHV2FA7ITVBQIARL3LALVU6DPELFIJRQDGK --rpc-url "https://rpc-futurenet.stellar.org:443" --network-passphrase "Test SDF Future Network ; October 2022")

echo $contract_id

soroban contract invoke --id $contract_id --source SBCKNWG5O3SRVA26LPWGBMHV2FA7ITVBQIARL3LALVU6DPELFIJRQDGK --rpc-url "https://rpc-futurenet.stellar.org:443" --network-passphrase "Test SDF Future Network ; October 2022" -- test