/*
 * radif
 * Copyright (C) 2025 - Luca Cireddu (IS0GVH) <sardylan@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program. If not, see <https://www.gnu.org/licenses/>.
 */
use clap::Parser;
use futures::{StreamExt, TryStreamExt};
use tokio::fs::File;
use tokio_util::compat::TokioAsyncReadCompatExt;

#[derive(Parser)]
#[command(name = "count_qso")]
#[command(about = "Count QSO records in ADIF file")]
struct Args {
    files: Vec<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    if args.files.is_empty() {
        eprintln!("No files given. Please provide at least one ADIF file.");
        std::process::exit(1);
    }

    let count = futures::stream::iter(args.files)
        .then(|file_name| async move {
            Ok((
                file_name.clone(),
                File::open(file_name)
                    .await
                    .map_err(|e| anyhow::anyhow!(e))?,
            ))
        })
        .and_then(|(file_name, file)| async move { Ok((file_name, file.compat())) })
        .and_then(|(file_name, file)| async move {
            Ok((
                file_name,
                radif::parse(file).await.map_err(|e| anyhow::anyhow!(e))?,
            ))
        })
        .try_fold(0usize, |acc, (file_name, adif)| async move {
            println!("{}: {}", file_name, &adif.qso_count());
            Ok::<usize, anyhow::Error>(acc + adif.qso_count())
        })
        .await?;

    println!("Total QSO count: {}", count);

    Ok(())
}
