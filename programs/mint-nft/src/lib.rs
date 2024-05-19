use anchor_lang::prelude::*;

declare_id!("Df7dRn17zyZ2oJkHEEChMHNm5zoWhz3pfykrN3Wv7GM2");

#[program]
pub mod mint_nft {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
