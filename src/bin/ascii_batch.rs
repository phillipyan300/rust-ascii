use anyhow::{Context, Result};
use image::ImageReader;
use rayon::prelude::*;
use std::{fs, path::{Path, PathBuf}};
use rust_ascii::{select_filter, select_ramp, image_to_ascii};

#[derive(Clone)]
struct Params {
    cols: u32,
    cell_aspect: f32,
    resizer_name: String,
    ramp: Vec<u8>,
}

fn list_pngs(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut v: Vec<_> = fs::read_dir(dir)
        .with_context(|| format!("reading directory {}", dir.display()))?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().and_then(|s| s.to_str()) == Some("png"))
        .collect();
    v.sort(); // relies on %06d naming
    Ok(v)
}

fn main() -> Result<()> {
    // Simple CLI via env args: ascii-batch [frames_dir] [out_dir] [cols] [cell_aspect] [resizer] [ramp]
    let args: Vec<String> = std::env::args().collect();
    let frames_dir = PathBuf::from(args.get(1).cloned().unwrap_or_else(|| "frames".into()));
    let out_dir    = PathBuf::from(args.get(2).cloned().unwrap_or_else(|| "out_txt".into()));
    let cols: u32  = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(160);
    let cell_aspect: f32 = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(2.0);
    let resizer_name = args.get(5).cloned().unwrap_or_else(|| "triangle".into());
    let ramp_spec    = args.get(6).cloned().unwrap_or_else(|| "basic".into());

    fs::create_dir_all(&out_dir).with_context(|| format!("creating {}", out_dir.display()))?;

    let _filter = select_filter(&resizer_name)?;
    let ramp = select_ramp(&ramp_spec)?;

    let params = Params { cols, cell_aspect, resizer_name, ramp };
    let frames = list_pngs(&frames_dir)?;
    if frames.is_empty() {
        anyhow::bail!("no PNG frames found in {}", frames_dir.display());
    }

    eprintln!("Converting {} frames → {} (parallel)…", frames.len(), out_dir.display());

    // Convert in parallel
    frames.par_iter().enumerate().try_for_each(|(i, path)| -> Result<()> {
        let img = ImageReader::open(path)
            .with_context(|| format!("open {}", path.display()))?
            .decode()
            .with_context(|| format!("decode {}", path.display()))?;

        let s = image_to_ascii(
            &img,
            params.cols,
            params.cell_aspect,
            select_filter(&params.resizer_name)?,
            &params.resizer_name,
            params.ramp.clone(),
        )?;

        let name = out_dir.join(format!("{:06}.txt", i + 1));
        fs::write(&name, s).with_context(|| format!("write {}", name.display()))?;
        Ok(())
    })?;

    eprintln!("✅ Done.");
    Ok(())
}
