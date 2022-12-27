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

pub async fn run() -> Result<()> {
    stdout()
        .execute(EnterAlternateScreen)?
        .execute(Clear(ClearType::Purge))?
        .execute(cursor::Hide)?;

    terminal::enable_raw_mode()?;

    Home::new().await?.run().await?;

    terminal::disable_raw_mode()?;

    stdout()
        .execute(LeaveAlternateScreen)?
        .execute(cursor::Show)?;

    Ok(())
}
