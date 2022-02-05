use anchor_lang::prelude::*;
use num_derive::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod titato {
    use super::*;
    pub fn initialize(ctx: Context<SetupGame>) -> ProgramResult {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetupGame<'info> {
    #[account(init, payer=player_one)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player_one: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(Default)]
pub struct Game {
    players: [Pubkey; 2],
    turn: u8,
    board: [[Option<Sign>; 3]; 3],
    state: GameState,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
    Active,
    Tie,
    Won { winner: Pubkey },
}

impl Default for GameState {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(
    AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq, Copy, FromPrimitive, ToPrimitive,
)]
pub enum Sign {
    X,
    O,
}
