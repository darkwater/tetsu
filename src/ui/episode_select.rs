use std::io::{stdout, Write};

use anyhow::{bail, Context, Result};
use crossterm::{
    cursor::MoveTo,
    event::{Event, EventStream, KeyCode},
    style::{PrintStyledContent, Stylize},
    terminal::{self, Clear, ClearType},
    QueueableCommand,
};
use futures::{StreamExt, TryStreamExt};
use itertools::Itertools;
use unicode_width::UnicodeWidthStr;

use super::{enter_alt_screen, leave_alt_screen};
use crate::{
    anidb::records::{Anime, Episode, File, Group},
    mpv::{Loadfile, LoadfileMode, Mpv, SetProperty, Stop},
};

pub struct EpisodeSelect {
    anime: Anime,
    episodes: Vec<EpisodeListing>,
    selected: usize,
}

struct EpisodeListing {
    episode: Episode,
    files: Vec<FileListing>,
}

struct FileListing {
    file: File,
    group: Group,
    paths_on_disk: Vec<String>,
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

        episodes.sort_by_cached_key(|Episode { epno, .. }| {
            let Some(idx) = epno.bytes().position(|b| b.is_ascii_digit()) else {
                return (epno.clone(), 0);
            };

            let (alpha, num) = epno.split_at(idx);
            let Ok(num) = num.parse() else {
                return (epno.clone(), 0);
            };

            (alpha.to_string(), num)
        });

        let mut listings = vec![];

        for episode in episodes {
            let queries = sqlx::query!(
                "SELECT f.json as fjson, g.json as gjson FROM files f
                INNER JOIN groups g ON f.gid = g.gid
                WHERE f.eid = ?",
                episode.eid
            )
            .fetch_all(db)
            .await?
            .into_iter()
            .map(|row| async move {
                let file: File =
                    serde_json::from_str(&row.fjson).context("Invalid record in database")?;
                let group: Group =
                    serde_json::from_str(&row.gjson).context("Invalid record in database")?;

                let paths_on_disk =
                    sqlx::query_scalar!("SELECT path FROM indexed_files WHERE fid = ?", file.fid)
                        .fetch_all(db)
                        .await?;

                Result::<_, anyhow::Error>::Ok(FileListing { file, group, paths_on_disk })
            });

            let files = tokio_stream::iter(queries)
                .buffer_unordered(10)
                .try_collect()
                .await?;

            listings.push(EpisodeListing { episode, files });
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

            let groups = episode.files.iter().map(|f| &f.group.name).join(", ");

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

    /// Start MPV, load all episodes as playlist, and start playing
    /// the selected episode.
    pub async fn play(&mut self) -> Result<()> {
        leave_alt_screen()?;

        let mut mpv = Mpv::new().await.context("mpv new")?;

        let files = self
            .episodes
            .iter()
            .flat_map(|e| e.files.first())
            .flat_map(|f| f.paths_on_disk.first())
            .cloned()
            .collect::<Vec<_>>();

        if files.is_empty() {
            bail!("No files found on disk");
        }

        mpv.request(Stop {}).await.context("mpv stop failed")?;

        for path in files {
            dbg!(mpv
                .request(Loadfile { path, mode: LoadfileMode::Append })
                .await
                .context("mpv loadfile failed")?);
        }

        mpv.request(SetProperty {
            name: "playlist-pos".into(),
            value: self.selected.into(),
        })
        .await
        .context("mpv set playlist-pos failed")?;

        mpv.wait().await.context("mpv wait failed")?;

        enter_alt_screen()?;

        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        self.display().await?;

        let mut stdin = EventStream::new();

        loop {
            self.display().await?;

            match stdin.next().await {
                Some(Ok(Event::Key(key))) => match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Char('j') | KeyCode::Down => {
                        self.selected = (self.selected + 1).min(self.episodes.len() - 1);
                    }
                    KeyCode::Char('k') | KeyCode::Up => {
                        self.selected = self.selected.saturating_sub(1);
                    }
                    KeyCode::Enter => {
                        self.play().await?;
                    }
                    ev => println!("{ev:?}"),
                },
                Some(Ok(Event::Resize(_, _))) => {}
                Some(Err(err)) => return Err(err.into()),
                None => break,
                ev => println!("{ev:?}"),
            }
        }

        Ok(())
    }
}
