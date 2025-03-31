use std::path::Path;

use anyhow::bail;

pub fn connect_sqlite3(path: &Path) -> anyhow::Result<rusqlite::Connection> {
	if !path.exists() {
		bail!(
			"Could not find a sqlite db in {:?}",
			path.canonicalize()
				.map(|e| e.to_string_lossy().into_owned())
				.unwrap_or_else(|_| path.to_string_lossy().into_owned())
		);
	}

	let cxn = rusqlite::Connection::open(path)?;

	Ok(cxn)
}
