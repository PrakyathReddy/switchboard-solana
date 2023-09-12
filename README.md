# switchboard-solana

Clone the Switchboard x Solana SDK to get started:
git clone https://github.com/switchboard-xyz/solana-sdk

Deployment:
Program ID: SW1TCH7qEPTdLsDHRgPuMQjbQxKdH2aBStViMFnt64f
Attestation Program ID: sbattyXrzedoNATfc4L31wC9Mhxsi1BmFhTiN8gDshx

Devnet:
Public Attestation Queue: CkvizjVnm2zA5Wuwan34NhVT3zFc7vqUyGnA6tuEF5aE
Public Oracle Queue: uPeRMdfPmrPqgRWSrjAnAkH78RqAhe5kXoW6vBYRqFX
Public Crank: UcrnK4w2HXCEjY8z6TcQ9tysYr3c9VcFLdYAU9YQP5e

Miami Weather feed address:
https://app.switchboard.xyz/solana/devnet/feed/6gx1DrVMMCtP2qebzHiHdzfoVQPLVDJHWXBFQJVyfBQ

1. Add the switchboard-solana crate to your Cargo.toml file:
[dependencies]
switchboard-solana = "0.9"

2. Create the ReadResult Accounts context containing the Switchboard data feed
Anchor provides the anchor-lang AccountLoader trait to:
    i. verify the account has the correct discriminator (all AggregatorAccounts share the same first 8 bytes)
    ii. the account is owned by the program ID defined in the switchboard-solana crate
The ReadResult Accounts context would look like:
use anchor_lang::prelude::*;
use switchboard_solana::{AggregatorAccountData};
#[derive(Accounts)]
pub struct ReadResult<'info> {
    pub switchboard_aggregator: AccountLoader<'info, AggregatorAccountData>,
}

3. Create the read_result instruction with the ReadResult Accounts context
Now lets add a read_result instruction to our program and pass in the ReadResult context.
First, we will deserialize the account data into the AggregatorAccountData (docs.rs) struct.
Next, we will use the TryInto trait to convert the SwitchboardDecimal (docs.rs) into the f64 primitive because it's easier to work with.

4. Submit a transaction on-chain with the read_result instruction
We will need to build a Solana transaction that contains our aggregator address to read.
