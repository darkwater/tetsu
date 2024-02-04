use std::io::{stdout, Write};

use anyhow::{Context, Result};
use crossterm::{
    cursor::MoveTo,
    event::{Event, EventStream, KeyCode},
    style::{PrintStyledContent, Stylize},
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};
use tokio_stream::StreamExt;
use unicode_width::UnicodeWidthStr;

use super::episode_select::EpisodeSelect;
use crate::anidb::records::Anime;

pub struct Home {
    anime: Vec<Anime>,
    selected: usize,
}

impl Home {
    pub async fn new() -> Result<Self> {
        let db = crate::DB.get().await;

        let mut anime = sqlx::query!("SELECT json FROM anime")
            .fetch_all(db)
            .await?
            .into_iter()
            .map(|row| serde_json::from_str(&row.json).context("Invalid record in database"))
            .collect::<Result<Vec<Anime>>>()?;

        anime.sort_by(|a, b| a.romaji_name.cmp(&b.romaji_name));

        Ok(Self { anime, selected: 0 })
    }

    pub async fn display(&self) -> Result<()> {
        let (width, height) = terminal::size()?;
        let width = width as usize;
        let height = height as usize;

        let mut stdout = stdout();

        stdout.queue(Clear(ClearType::All))?.queue(MoveTo(0, 0))?;

        let max = self.anime.len().saturating_sub(height);
        let start = self.selected.saturating_sub(height / 2).min(max);

        for (i, anime) in self.anime.iter().skip(start).take(height).enumerate() {
            let mut title = anime.romaji_name.as_str();

            if title.width() > width {
                title = &title[..width];
            }

            stdout
                .queue(MoveTo(0, i as u16))?
                .queue(PrintStyledContent(if i == self.selected - start {
                    title.black().bold().on_blue()
                } else {
                    title.blue()
                }))?;
        }

        stdout.flush()?;

        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        let mut stdin = EventStream::new();

        loop {
            self.display().await?;

            match stdin.next().await {
                Some(Ok(Event::Key(key))) => match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char('j') | KeyCode::Down => {
                        self.selected = (self.selected + 1).min(self.anime.len() - 1);
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        self.selected = self.selected.saturating_sub(1);
                    }
                    KeyCode::Enter => {
                        let anime = self.anime[self.selected].clone();
                        EpisodeSelect::new(anime).await?.run().await?;
                    }
                    ev => println!("{ev:?}"),
                },
                Some(Err(err)) => return Err(err.into()),
                None => break,
                ev => println!("{ev:?}"),
            }
        }

        Ok(())
    }
}
