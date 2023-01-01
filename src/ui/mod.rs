use std::io::stdout;

use anyhow::Result;
use crossterm::{
    cursor,
    terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

use self::home::Home;

mod episode_select;
mod home;

pub fn enter_alt_screen() -> Result<()> {
    stdout()
        .execute(EnterAlternateScreen)?
        .execute(Clear(ClearType::Purge))?
        .execute(cursor::Hide)?;

    terminal::enable_raw_mode()?;

    Ok(())
}

pub fn leave_alt_screen() -> Result<()> {
    terminal::disable_raw_mode()?;

    stdout()
        .execute(LeaveAlternateScreen)?
        .execute(cursor::Show)?;

    Ok(())
}

pub async fn run() -> Result<()> {
    let mut home = Home::new().await?;

    enter_alt_screen()?;

    let res = home.run().await;

    leave_alt_screen()?;

    res
}
