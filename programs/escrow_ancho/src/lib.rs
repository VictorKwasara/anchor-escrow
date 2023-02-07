use anchor_lang::prelude::*;

declare_id!("C4Nz5x37jYuAvdvoJLKbxLkPmNdz5QiyWbxvsAq4c5s9");

#[program]
pub mod escrow_ancho {
    use super::*;

    pub fn initialize
    (ctx: Context<Initialize>,
     _vault_account_bump:u8,
     initializer_amount: u64,
     taker_amount: u64,
    ) -> Result<()> {

        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel>) ->ProgramResult {
        Ok(())
    }

    pub fn exchange(ctx: Context<Exchange>) -> Result<()>{
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
