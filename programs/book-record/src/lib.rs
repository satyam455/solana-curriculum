//- Create a library program that manages book records with different data types

use anchor_lang::prelude::*;

declare_id!("9auMpKHJfh7CcagQeg4wWoqJSdQHbjNAfHttLhZp9FGK");

#[program]
pub mod book_record {
    use super::*;

    pub fn initialize_record(
        ctx: Context<InitializeRecord>, username: String, bookname: String, email: String, address: String, book_id: u64, borrow_type: BorrowType
    ) -> Result<()> {
       
        let library_ = &mut ctx.accounts.library;

        let clock = Clock::get()?;

        library_.owner = ctx.accounts.owner.key();
        library_.book_id = book_id;
        library_.username = username;
        library_.bookname = bookname;
        library_.borrowed_at = clock.unix_timestamp;
        library_.returned_at = None;

        library_.contact = UserContact {
            borrowtype: borrow_type,
            email: email,
            address: address,
        };
    
        Ok(())
    }

    pub fn return_book(ctx: Context<ReturnBook>) -> Result<()> {
        let record = &mut ctx.accounts.record;
        let clock = Clock::get()?;

        record.returned_at = Some(clock.unix_timestamp);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(book_id: u64)] //taking book_id as an unique for pda books id will be always unique for every book
pub struct InitializeRecord<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + BookRecords::INIT_SPACE,
        seeds = [b"library", owner.key().as_ref(), &book_id.to_le_bytes()],
        bump,
    )]
    pub library: Account<'info, BookRecords>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}


impl BookRecords {
    pub const MAX_USERNAME: usize = 50;
    pub const MAX_BOOK_NAME: usize = 100;
    pub const MAX_EMAIL: usize = 100;
    pub const MAX_ADDRESS: usize = 100;


   pub const CONTACT_SPACE: usize = 
   1 +
   4 + Self::MAX_EMAIL +
   4 + Self::MAX_ADDRESS;

   
       pub const INIT_SPACE: usize =
        32 + 8 + 4 + Self::MAX_USERNAME + 4 + Self::MAX_BOOK_NAME + 1 + 4 + 8 + 4;
}

#[derive(Accounts)]
pub struct ReturnBook<'info> {
    #[account(
        mut,
        has_one = owner      
    )]
    pub record: Account<'info, BookRecords>,
    pub owner: Signer<'info>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub enum BorrowType {
    InLibrary,
    TakeHome,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UserContact {
    pub borrowtype: BorrowType,
    pub email: String,
    pub address: String,
}

#[account]
pub struct BookRecords {
    pub owner: Pubkey,
    pub book_id: u64,
    pub username: String,
    pub bookname: String,
    pub contact: UserContact,
    pub borrowed_at: i64,
    pub returned_at: Option<i64>,
    pub bump: u8,
}

/*
manages book record means

book record will be of 2 types
1) users who are reading the booking in the library only
2) users who are taking the booking with them from the library

who came in the library >> record name and timing of entry

The user came in the library has taken any book with him
if YES the record will be registered in 2nd book
        >> record its contact (email/phn number/address some id) and book name he is taking
        >> time of taken and some small security fees given to the library.

        >> during return of book security fee will be given back and signature will be take
        >> time of returning date and day of returning the book

if NO, record will be registered on 1st book ,
       >> user name/ entry timing/ contact/ address/ some id
       >> during exit the signature will be taken and recorded in the 1st book timing of exit

*/
