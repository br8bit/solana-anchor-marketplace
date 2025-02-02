# NFT marketplace built on Solana blockchain

This project showcases the creation of a simple NFT marketplace using the Token Metadata Program. Within this marketplace, users have the ability to list their NFTs for sale, remove them from the listing, and purchase NFTs listed by other users. Additionally, the marketplace incorporates features such as transaction fees and a treasury system.

### The key features of this NFT marketplace program include:

- **Listing NFTs for Sale**: Users can add their NFTs to the marketplace, specifying the price they wish to sell them for.
- **Delisting NFTs**: Users can remove their NFTs from the marketplace if they no longer wish to sell them.
- **Purchasing Listed NFTs**: Buyers can acquire NFTs listed on the marketplace by transferring the specified price to the seller.
- **Marketplace Fees**: The marketplace charges a small fee for each transaction, which is then deposited into a treasury account.
- **Treasury System**: The marketplace maintains a treasury account that collects the fees generated from transactions. This treasury can be utilized for various purposes, such as platform maintenance or community initiatives.

This example provides a solid foundation for building an NFT marketplace on the Solana blockchain, leveraging the capabilities of the Token Metadata Program. Feel free to explore and build upon this example to suit your specific requirements.

## Program Structure and Entry Points

The program exposes four main entry points in the `anchor_marketplace` module:

- `initialize(ctx: Context<Initialize>, name: String, fee: u16)`:

  - Creates a new marketplace instance with the specified name and fee structure
  - Sets up the treasury and rewards system

- `listing(ctx: Context<List>, price: u64)`:

  - Creates a new NFT listing with the specified price
  - Transfers the NFT to the vault

- `delist(ctx: Context<Delist>)`:

  - Removes an NFT listing
  - Returns the NFT to the original owner

- `purchase(ctx: Context<Purchase>)`:
  - Processes the purchase of a listed NFT
  - Handles the SOL transfer, including marketplace fees
  - Transfers the NFT to the buyer
  - Cleans up the listing accounts
