# Auction House Agreement

This program manages requests and approvals for listing NFTs minted by a wallet for a Metaplex Auction House operated by Holaplex.

## Accounts

### Agreement

```rust
pub struct Agreement {
    pub authorizer: Pubkey,
    pub auction_house: Pubkey,
    pub approved: bool,
}
```

An agreement represents a request to list a craetor's work on a auction house. The creator can then approve the agreeement to grant the auction house access to display their NFTs for sale. In the near future the authorizers will be expanded to Metaplex Collection authorities.

