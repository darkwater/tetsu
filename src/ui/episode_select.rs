use std::io::{stdout, Write};

use anyhow::{Context, Result};
use crossterm::{
    cursor::MoveTo,
    event::{Event, EventStream, KeyCode},
    style::{PrintStyledContent, Stylize},
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};
use itertools::Itertools;
use tokio_stream::StreamExt;
use unicode_width::UnicodeWidthStr;

use crate::anidb::records::{Anime, Episode, File, Group};

pub struct EpisodeSelect {
    anime: Anime,
    episodes: Vec<EpisodeListing>,
    selected: usize,
}

struct EpisodeListing {
    episode: Episode,
    files_groups: Vec<(File, Group)>,
}

impl EpisodeSelect {
    pub async fn new(anime: Anime) -> Result<Self> {
        let db = crate::DB.get().await;

        let mut episodes = sqlx::query!("SELECT json FROM episodes WHERE aid = ?", anime.aid)
            .fetch_all(db)
            .await?
            .into_iter()
            .map(|row| serde_json::from_str(&row.json).context("Invalid record in database"))
            .collect::<Result<Vec<Episode>>>()?;

        episodes.sort_by(|a, b| a.epno.cmp(&b.epno));

        let mut listings = vec![];

        for episode in episodes {
            let files_groups = sqlx::query!(
                "SELECT f.json as fjson, g.json as gjson FROM files f
                INNER JOIN groups g ON f.gid = g.gid
                WHERE f.eid = ?",
                episode.eid
            )
            .fetch_all(db)
            .await?
            .into_iter()
            .map(|row| {
                let file =
                    serde_json::from_str(&row.fjson).context("Invalid record in database")?;
                let group =
                    serde_json::from_str(&row.gjson).context("Invalid record in database")?;

                Ok((file, group))
            })
            .collect::<Result<Vec<(File, Group)>>>()?;

            listings.push(EpisodeListing { episode, files_groups });
        }

        Ok(Self {
            anime,
            episodes: listings,
            selected: 0,
        })
    }

    pub async fn display(&self) -> Result<()> {
        let (width, height) = terminal::size()?;

        let mut stdout = stdout();

        stdout
            .queue(Clear(ClearType::All))?
            .queue(MoveTo(0, 0))?
            .queue(PrintStyledContent(self.anime.romaji_name.as_str().blue()))?;

        for (i, episode) in self.episodes.iter().enumerate().take(height as usize) {
            let selected = i == self.selected;

            let groups = episode.files_groups.iter().map(|(_, g)| &g.name).join(", ");

            let mut title = format!(
                "{}. {}  {}",
                episode.episode.epno,
                if selected {
                    episode.episode.romaji.as_str().on_blue()
                } else {
                    episode.episode.romaji.as_str().blue()
                },
                groups.dark_grey(),
            );

            if title.width() > width as usize {
                title.truncate(width as usize);
            }

            stdout
                .queue(MoveTo(0, 2 + i as u16))?
                .queue(PrintStyledContent(title.stylize()))?;
        }

        stdout.flush()?;

        Ok(())
    }

    pub async fn play(&mut self) -> Result<()> {
        todo!()
    }

    pub async fn run(&mut self) -> Result<()> {
        self.display().await?;

        let mut stdin = EventStream::new();

        loop {
            match stdin.next().await {
                Some(Ok(Event::Key(key))) => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('j') | KeyCode::Down => {
                        self.selected = (self.selected + 1).min(self.episodes.len() - 1);
                        self.display().await?;
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        self.selected = self.selected.saturating_sub(1);
                        self.display().await?;
                    }
                    KeyCode::Enter => {
                        self.play().await?;
                    }
                    ev => println!("{ev:?}"),
                },
                Some(Ok(Event::Resize(_, _))) => self.display().await?,
                Some(Err(err)) => return Err(err.into()),
                None => break,
                ev => println!("{ev:?}"),
            }
        }

        Ok(())
    }
}
