use std::io::prelude::*;
use zip::*;
use std::io;
use std::fs::File;
use std::str;
use std::fs;

pub fn unzip(file: &File, decompressed_musicxml_bytes: &mut Vec<u8>) -> zip::result::ZipResult<()> {
  let mut archive = zip::ZipArchive::new(file)?;

  for i in 0..archive.len() {
    let mut file = archive.by_index(i).unwrap();
    let outpath = file.sanitized_name();

    /*
      Compressed .mxl files are ZIP archives with one meta file identifying the path to the main XML score. (ref: https://www.musicxml.com/tutorial/compressed-mxl-files/zip-archive-structure for more details.)

      We'll just return the first XML file that isn't the path META-INF/container.xml. This isn't robust but we can improve it later.
     */
    if outpath.to_str().unwrap() != "META-INF\\container.xml" {
      trace!("Detected non-container.xml XML file \"{}\" ({} bytes)", outpath.as_path().display(), file.size());
      file.read_to_end(decompressed_musicxml_bytes).unwrap();

      break;
    }
  }

  Ok(())
}

#[cfg(test)]
mod tests {
  // Note this useful idiom: importing names from outer (for mod tests) scope.
  use super::*;

  fn setup() -> Result<(), log::SetLoggerError> {
    return Ok(fern::Dispatch::new()
      .format(|out, message, record| {
          out.finish(format_args!(
              "[{}][{}] {}",
              record.level(),
              record.target(),
              message,
          ))
      })
      .chain(
          fern::Dispatch::new()
              .level(log::LevelFilter::Trace)
              .chain(io::stdout()),
      )
      .apply()?);
  }

  #[test]
  fn test_unzip() {
    setup().unwrap();

    let file = File::open("./test_assets/etude_no._1.mxl").unwrap();
    let mut decompressed_bytes = Vec::<u8>::new();
    unzip(&file, &mut decompressed_bytes).unwrap();
    let score = str::from_utf8(&decompressed_bytes[..]).unwrap();

    assert!(score.contains("<work-title>Etude No. 1</work-title>"));
  }
}
