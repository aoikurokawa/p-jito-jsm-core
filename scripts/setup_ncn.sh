#!/usr/bin/bash
set -x

# Initialize Vault
cargo r -- vault vault initialize Sy2gWQkAHHSK5jDSebSGS1ZvTPX1cDU66GZrr8apckf 100 100 100 9 --rpc-url "https://api.devnet.solana.com" --keypair ~/.config/solana/id.json
# cargo r -- vault vault initialize --rpc-url "https://api.devnet.solana.com" --keypair ~/.config/solana/id.json --token-mint Sy2gWQkAHHSK5jDSebSGS1ZvTPX1cDU66GZrr8apckf --deposit-fee-bps 100 --withdraw-fee-bps 100 -reward-fee-bps 100 --decimals 9
#
#  cargo r -- vault vault initialize-ncn-vault-slasher-ticket 6D4e44djPyHih4ne361s91it6vWq5sxVyxURkzpB72c9 7CrmCmuf2GQzkuhUvCAuAanJBaZ5HpVCw1VkqTCo4AG7 7CrmCmuf2GQzkuhUvCAuAanJBaZ5HpVCw1VkqTCo4AG7   --rpc-url https://api.devnet.solana.com --keypair /home/aoi/.config/solana/id.json
