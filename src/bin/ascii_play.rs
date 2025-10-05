use anyhow::{Context, Result};
use crossterm::{
    cursor::MoveTo,
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, Clear, ClearType},
};
use std::{
    fs,
    io::{stdout, Write},
    path::{Path, PathBuf},
    time::{Duration, Instant},
};

fn list_txts(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut v: Vec<_> = fs::read_dir(dir)
        .with_context(|| format!("reading {}", dir.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("txt"))
        .collect();
    v.sort();
    Ok(v)
}

fn main() -> Result<()> {
    // ascii-play [txt_dir] [fps]
    let args: Vec<String> = std::env::args().collect();
    let txt_dir = PathBuf::from(args.get(1).cloned().unwrap_or_else(|| "out_txt".into()));
    let fps: u32 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(30);

    let frames = list_txts(&txt_dir)?;
    if frames.is_empty() {
        anyhow::bail!("no .txt frames found in {}", txt_dir.display());
    }

    let frame_time = Duration::from_millis((1000 / fps).max(1) as u64);
    let mut out = stdout();

    execute!(out, EnterAlternateScreen)?;
    let start = Instant::now();

    for (i, path) in frames.iter().enumerate() {
        let ascii = fs::read_to_string(path)
            .with_context(|| format!("read {}", path.display()))?;
        execute!(out, Clear(ClearType::All), MoveTo(0, 0))?;
        out.write_all(ascii.as_bytes())?;
        out.flush()?;

        // maintain cadence
        let deadline = start + frame_time * (i as u32 + 1);
        let now = Instant::now();
        if deadline > now {
            std::thread::sleep(deadline - now);
        }
    }

    execute!(out, LeaveAlternateScreen)?;
    Ok(())
}
