use std::{fs::File, path::Path};

use anyhow::{Context, Result};
use indicatif::ProgressBar;
use md4::{Digest, Md4};
use memmap::Mmap;
use rayon::{prelude::ParallelIterator, slice::ParallelSlice};

const ED2K_CHUNK_SIZE: usize = 9728000;

pub fn hash_file<P: AsRef<Path>>(file: P, pb: &ProgressBar) -> Result<[u8; 16]> {
    let file = File::open(file).context("Failed to open file")?;
    let map = unsafe { Mmap::map(&file) }.context("Failed to map file into memory")?;

    pb.set_length(map.len() as u64 / ED2K_CHUNK_SIZE as u64);

    let hashes: Vec<[u8; 16]> = map
        .par_chunks(ED2K_CHUNK_SIZE)
        .map(Md4::digest)
        .map(Into::into)
        .inspect(|_| pb.inc(1))
        .collect();

    let root_hash = Md4::digest(hashes.concat());

    Ok(root_hash.into())
}
