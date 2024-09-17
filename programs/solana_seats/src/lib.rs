use anchor_lang::prelude::*;

declare_id!("FsdnUyKAjp836EWPNyY7iwNvpTnNW1sdzTSzq6d16Nwj");

#[program]
pub mod solana_seats {
    use super::*;

    pub fn create_event(ctx: Context<CreateEvent>, total_tickets: u64) -> Result<()> {
        let event = &mut ctx.accounts.event;
        event.organizer = *ctx.accounts.organizer.key;
        event.total_tickets = total_tickets;
        event.tickets_sold = 0;
        Ok(())
    }

    pub fn buy_ticket(ctx: Context<BuyTicket>) -> Result<()> {
        let event = &mut ctx.accounts.event;
        let user = &mut ctx.accounts.user;

        require!(
            event.tickets_sold < event.total_tickets,
            TicketError::SoldOut
        );

        event.tickets_sold += 1;
        user.ticket_owned += 1;

        Ok(())
    }

    pub fn resell_ticket(ctx: Context<ResellTicket>, resale_price: u64) -> Result<()> {
        let user = &mut ctx.accounts.user;

        require!(user.ticket_owned > 0, TicketError::NoTickets);

        user.ticket_owned -= 1;
        let platform_fee = (resale_price as f64 * 0.05) as u64; // 5% fee
        let seller_cut = resale_price - platform_fee;
        println!("seller cut: {:?}", seller_cut);
        // Transfer resale funds (pseudocode - implement actual token transfer)
        // transfer_to_seller(seller_cut);
        // transfer_to_platform(platform_fee);

        Ok(())
    }
}

#[account]
pub struct Event {
    pub organizer: Pubkey,
    pub total_tickets: u64,
    pub tickets_sold: u64,
}

#[account]
pub struct User {
    pub ticket_owned: u64,
}

#[error_code]
pub enum TicketError {
    #[msg("All tickets are sold out.")]
    SoldOut,
    #[msg("You don't have any tickets to resell.")]
    NoTickets,
    #[msg("Unauthorized User Signer")]
    Unauthorized,
}

#[derive(Accounts)]
pub struct CreateEvent<'info> {
    #[account(init, payer = organizer, space = 8 + 32 + 8 + 8)]
    pub event: Account<'info, Event>,
    #[account(mut)]
    pub organizer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct BuyTicket<'info> {
    #[account(mut)]
    pub event: Account<'info, Event>,
    #[account(mut)]
    pub user: Account<'info, User>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ResellTicket<'info> {
    #[account(mut)]
    pub user: Account<'info, User>,
}
