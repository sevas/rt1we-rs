//! Persists rendered images and their render parameters to disk, so past
//! renders survive an app restart and can be browsed from the GUI.
use rt1we_renderer::image::ImageRGBA;
use rt1we_renderer::ppmio::{ppmread, ppmwrite};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

const THUMB_MAX_DIM: usize = 96;

#[derive(Debug, Clone)]
pub struct RenderMeta {
    pub width: usize,
    pub height: usize,
    pub max_depth: usize,
    pub samples_per_pixel: usize,
    pub use_parallel: bool,
    pub render_ms: u128,
}

#[derive(Debug, Clone)]
pub struct HistoryEntry {
    /// Milliseconds since the Unix epoch when the render was saved; doubles as a
    /// sortable, collision-resistant id and as the filename stem on disk.
    pub id: u128,
    pub meta: RenderMeta,
    pub image_path: PathBuf,
    pub thumb_path: PathBuf,
}

pub fn ensure_history_dir() -> PathBuf {
    let dir = PathBuf::from("history");
    let _ = fs::create_dir_all(&dir);
    dir
}

pub fn save_render(dir: &Path, image: &ImageRGBA, meta: RenderMeta) -> HistoryEntry {
    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();

    let image_path = dir.join(format!("{id}.ppm"));
    let thumb_path = dir.join(format!("{id}_thumb.ppm"));
    let meta_path = dir.join(format!("{id}.meta"));

    ppmwrite(image_path.to_str().expect("invalid path"), image);
    ppmwrite(thumb_path.to_str().expect("invalid path"), &make_thumbnail(image, THUMB_MAX_DIM));
    fs::write(&meta_path, meta_to_string(&meta)).expect("unable to write render metadata");

    HistoryEntry { id, meta, image_path, thumb_path }
}

/// Scan the history directory for saved renders, newest first. Entries with
/// missing or unreadable metadata/image files are silently skipped.
pub fn load_history(dir: &Path) -> Vec<HistoryEntry> {
    let mut entries = Vec::new();

    let Ok(read_dir) = fs::read_dir(dir) else { return entries };
    for dir_entry in read_dir.flatten() {
        let meta_path = dir_entry.path();
        if meta_path.extension().and_then(|e| e.to_str()) != Some("meta") {
            continue;
        }
        let Some(id) = meta_path.file_stem().and_then(|s| s.to_str()).and_then(|s| s.parse::<u128>().ok())
        else {
            continue;
        };
        let Ok(contents) = fs::read_to_string(&meta_path) else { continue };
        let Some(meta) = meta_from_str(&contents) else { continue };

        let image_path = dir.join(format!("{id}.ppm"));
        let thumb_path = dir.join(format!("{id}_thumb.ppm"));
        if !image_path.exists() || !thumb_path.exists() {
            continue;
        }

        entries.push(HistoryEntry { id, meta, image_path, thumb_path });
    }

    entries.sort_by(|a, b| b.id.cmp(&a.id));
    entries
}

pub fn load_image(path: &Path) -> ImageRGBA {
    ppmread(path.to_str().expect("invalid path"))
}

fn make_thumbnail(im: &ImageRGBA, max_dim: usize) -> ImageRGBA {
    let scale = (max_dim as f32 / im.width.max(im.height) as f32).min(1.0);
    let tw = ((im.width as f32 * scale).round() as usize).max(1);
    let th = ((im.height as f32 * scale).round() as usize).max(1);

    let mut thumb = ImageRGBA::new(tw, th);
    for j in 0..th {
        for i in 0..tw {
            let src_i = (i * im.width / tw).min(im.width - 1);
            let src_j = (j * im.height / th).min(im.height - 1);
            thumb.put_u32(i, j, im.at_u32(src_i, src_j));
        }
    }
    thumb
}

fn meta_to_string(meta: &RenderMeta) -> String {
    format!(
        "width={}\nheight={}\nmax_depth={}\nsamples_per_pixel={}\nuse_parallel={}\nrender_ms={}\n",
        meta.width, meta.height, meta.max_depth, meta.samples_per_pixel, meta.use_parallel, meta.render_ms
    )
}

fn meta_from_str(s: &str) -> Option<RenderMeta> {
    let fields: HashMap<&str, &str> =
        s.lines().filter_map(|line| line.split_once('=')).map(|(k, v)| (k.trim(), v.trim())).collect();

    Some(RenderMeta {
        width: fields.get("width")?.parse().ok()?,
        height: fields.get("height")?.parse().ok()?,
        max_depth: fields.get("max_depth")?.parse().ok()?,
        samples_per_pixel: fields.get("samples_per_pixel")?.parse().ok()?,
        use_parallel: fields.get("use_parallel")?.parse().ok()?,
        render_ms: fields.get("render_ms")?.parse().ok()?,
    })
}

#[cfg(test)]
mod test {
    use super::*;
    use std::env;

    fn temp_history_dir(name: &str) -> PathBuf {
        let dir = env::temp_dir().join(name);
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn test_save_and_load_history_roundtrip() {
        let dir = temp_history_dir("rt1we-gui_history_test_roundtrip");

        let mut im = ImageRGBA::new(8, 4);
        // ppmio's PPM format carries no alpha channel, so use alpha=255 to
        // roundtrip cleanly (see rt1we_renderer::ppmio docs).
        im.put_u32(1, 1, 0x112233FF);
        let meta = RenderMeta {
            width: 8,
            height: 4,
            max_depth: 12,
            samples_per_pixel: 34,
            use_parallel: true,
            render_ms: 567,
        };

        let saved = save_render(&dir, &im, meta);

        let loaded = load_history(&dir);
        assert_eq!(loaded.len(), 1);
        assert_eq!(loaded[0].id, saved.id);
        assert_eq!(loaded[0].meta.width, 8);
        assert_eq!(loaded[0].meta.height, 4);
        assert_eq!(loaded[0].meta.max_depth, 12);
        assert_eq!(loaded[0].meta.samples_per_pixel, 34);
        assert_eq!(loaded[0].meta.use_parallel, true);
        assert_eq!(loaded[0].meta.render_ms, 567);

        let im_reloaded = load_image(&loaded[0].image_path);
        assert_eq!(im_reloaded.width, im.width);
        assert_eq!(im_reloaded.height, im.height);
        assert_eq!(im_reloaded.pixels, im.pixels);
    }

    #[test]
    fn test_load_history_sorts_newest_first_and_skips_incomplete_entries() {
        let dir = temp_history_dir("rt1we-gui_history_test_sort");

        let im = ImageRGBA::new(2, 2);
        let meta = |ms| RenderMeta {
            width: 2,
            height: 2,
            max_depth: 1,
            samples_per_pixel: 1,
            use_parallel: false,
            render_ms: ms,
        };

        let first = save_render(&dir, &im, meta(1));
        std::thread::sleep(std::time::Duration::from_millis(2));
        let second = save_render(&dir, &im, meta(2));

        // A stray metadata file with no matching image should be ignored.
        fs::write(dir.join("999999999999999.meta"), "width=1\nheight=1\n").unwrap();

        let loaded = load_history(&dir);
        assert_eq!(loaded.len(), 2);
        assert_eq!(loaded[0].id, second.id);
        assert_eq!(loaded[1].id, first.id);
    }
}
