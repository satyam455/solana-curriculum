// #![allow(clippy::result_large_err)]

//Build a complete note-taking program with CRUD operations

use anchor_lang::prelude::*;

declare_id!("GnoaMeqDyteiABnVA6h9KXXPPTAbimFCBNyy9aS8LWYn");

#[program]
pub mod note_taking {
    use super::*;

    pub fn initialize_data(ctx: Context<InitializeData>, title: String, message: String) -> Result<()> {
        let data = &mut ctx.accounts.data;
        
        data.owner = ctx.accounts.owner.key();
        data.title = title;
        data.message = message;
        Ok(())
    }

    pub fn update_data(ctx: Context<UpdateData>, title: String, message: String) -> Result<()> {

        let data = &mut ctx.accounts.data;

        data.title = title;
        data.message = message;
        
        Ok(())

    }

    pub fn delete_data(ctx: Context<DeleteData>) -> Result<()> {
        Ok(())

    }
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct InitializeData<'info> {
    #[account(
        init,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        space = Data::INIT_SPACE,
        payer = owner
    )]
    pub data: Account<'info, Data>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct UpdateData<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        realloc = 8 + Data::INIT_SPACE,
        realloc::payer = owner,
        realloc::zero = true, 
    )]
    pub data: Account<'info, Data>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteData<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), owner.key().as_ref()],
        bump,
        close = owner,
    )]
    pub data: Account<'info, Data>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct Data {
    pub owner: Pubkey,
    #[max_len(50)]
    pub title: String,
    #[max_len(250)]
    pub message: String,

}

// #[derive(Accounts)]
// pub struct Updatedata<'info> {

// }


