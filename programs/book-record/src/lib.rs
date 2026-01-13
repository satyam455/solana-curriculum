//- Create a library program that manages book records with different data types

use anchor_lang::prelude::*;

declare_id!("9auMpKHJfh7CcagQeg4wWoqJSdQHbjNAfHttLhZp9FGK");

#[program]
pub mod book_record {
    use super::*;

    pub fn initialize(
        ctx: Context<InitializeRecord>, /*, username: String, bookname: String, email: String, mobile: u8, address: String */
    ) -> Result<()> {
        /*
        pub library = ctx.accounts.library;

        let clock = Clock::get()?;

        library.owner = ctx.accounts.owner.key();

        library.username = username;
        library.bookname = bookname;

        library.contact = UserContact {
        taken: true,
        email: email,
        mobile: mobile,
        address: address,
        };

        if library.contact.taken == true {

        pay some sol to library program and also add the logic when the book get returned it get back to the user
        >> cpi will gonna implement here which like 0.001SOL from user to library when a book is taken and will return back when the user returned the book


        }




                 */
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(username: String,bookname: String)]
/*#[instruction(email:String, mobile: u8, address: String)] */
pub struct InitializeRecord<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + BookRecords::INIT_SPACE,
        seeds = [b"library", owner.key().as_ref()],
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

    pub const INIT_SPACE: usize =
        32 + 4 + Self::MAX_USERNAME + 4 + Self::MAX_BOOK_NAME + 1 + 4 + 8 + 4;
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct UserContact {
    pub taken: bool,
    pub email: String,
    pub mobile: u8,
    pub address: String,
}

#[account]
pub struct BookRecords {
    pub username: String,
    pub bookname: String,
    pub contact: UserContact,
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
