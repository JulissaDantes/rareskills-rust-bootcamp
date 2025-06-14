## Report for soroban NFT contract

### Description
The contract.rs containing the `NFTToken` contract contains of a standard implementation of a non fungible token, for Soroban, the smart contract platform for Stellar.

### Findings

#### High Severity
1. No access control on `burn` function
\t Anyone can burn any token, including tokens the caller might not own.

2. `transfer_from` doesn't modify approval
\t If the transfer is performed not by the owner, but an approved account, after performing the transfer the approval is not revoked, therefore said approved account could transfer the same token several times.

#### Medium Severity
None

#### Low Severity
1. Unclear supply type
\t When minting a new token with `mint_new` the supply is always incremented by 1, but depending on the size of supply it could overflow and start assigning existing tokens to different owners.