use crate::ledger::LedgerHash;
use rust_decimal::Decimal;
use std::{
    path::{Path, PathBuf},
    time::Duration,
};

/// Pretty print duration for use in logs.
///
/// Example: 1d 2h 3m 4s
pub fn pretty_print_duration(duration: Duration) -> String {
    let seconds = duration.as_secs();
    if seconds == 0 {
        return "0s".to_string();
    }

    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    let secs = seconds % 60;
    let mut parts = Vec::new();

    if days > 0 {
        parts.push(format!("{}d", days));
    }
    if hours > 0 {
        parts.push(format!("{}h", hours));
    }
    if minutes > 0 {
        parts.push(format!("{}m", minutes));
    }
    if secs > 0 {
        parts.push(format!("{}s", secs));
    }

    parts.join(" ")
}

/// Converts Nanomina to Mina, strips any trailing zeros, and converts -0 to 0.
/// This function takes a value in Nanomina, converts it to Mina by adjusting
/// the scale, normalizes the decimal representation to remove trailing zeros,
/// and converts any `-0` representation to `0`.
///
/// # Arguments
///
/// * `nanomina` - The amount in Nanomina to be converted.
///
/// # Returns
///
/// A `String` representing the value in Mina with trailing zeros removed.
pub fn nanomina_to_mina(nanomina: u64) -> String {
    let mut dec = Decimal::from(nanomina);
    dec.set_scale(9).unwrap();
    dec.normalize().to_string()
}

/// Calculate the total size of the file paths
pub fn calculate_total_size(paths: &[PathBuf]) -> u64 {
    paths.iter().fold(0, |acc, p| {
        p.metadata().map_or(acc, |metadata| acc + metadata.len())
    })
}

/// Extract the height and state hash from a PCB path
pub fn extract_height_and_hash(path: &Path) -> (u32, &str) {
    let filename = path
        .file_stem()
        .and_then(|x| x.to_str())
        .expect("Failed to extract filename from path");

    let mut parts = filename.split('-');

    match (parts.next(), parts.next(), parts.next()) {
        (Some(_), Some(height_str), Some(hash_part)) => {
            let block_height = height_str
                .parse::<u32>()
                .expect("Failed to parse block height");
            let hash = hash_part
                .split('.')
                .next()
                .expect("Failed to parse the hash");
            (block_height, hash)
        }
        _ => panic!("Filename format is invalid {}", filename),
    }
}

/// Extract the network, height/epoch, and hash from a PCB or staking ledger
/// path
pub fn extract_network_height_hash(path: &Path) -> (&str, u32, &str) {
    let filename = path
        .file_stem()
        .and_then(|x| x.to_str())
        .expect("Failed to extract filename from path");

    let mut parts = filename.split('-');

    match (parts.next(), parts.next(), parts.next()) {
        (Some(network), Some(height_or_epoch), Some(hash)) => {
            let height_or_epoch = height_or_epoch
                .parse::<u32>()
                .expect("Failed to parse height/epoch");
            let hash = hash.split('.').next().expect("Failed to parse the hash");
            (network, height_or_epoch, hash)
        }
        _ => panic!("Filename format is invalid {}", filename),
    }
}

/// Extract the epoch and ledger hash from a staking ledger path
pub fn extract_epoch_hash(path: &Path) -> (u32, LedgerHash) {
    let (epoch, hash) = extract_height_and_hash(path);
    (epoch, LedgerHash::new_or_panic(hash.to_string()))
}

/// Checks if the file name is valid for the provided hash validator
pub fn is_valid_file_name<P>(path: P, hash_validator: &dyn Fn(&str) -> bool) -> bool
where
    P: AsRef<Path>,
    P: Into<PathBuf>,
{
    let mut file_stem = path.as_ref().file_stem().and_then(|stem| stem.to_str());

    // check extension
    if let Some(ext) = path.as_ref().extension().and_then(|ext| ext.to_str()) {
        if ext == "gz" {
            // gzip compressed json
            if let Some((stem, ext)) = file_stem.as_ref().map(|file| {
                let mut parts = file.split('.');
                (parts.next().unwrap(), parts.next().unwrap())
            }) {
                if ext != "json" {
                    return false;
                }

                file_stem = Some(stem);
            }
        } else if ext != "json" {
            // uncompressed
            return false;
        }
    } else {
        return false;
    }

    // check validity of file stem
    file_stem.is_some_and(|stem| {
        let parts: Vec<&str> = stem.split('-').collect();

        // mainnet-<number>-<hash>.json
        if let [_, epoch_str, hash] = parts.as_slice() {
            epoch_str.parse::<u32>().is_ok() && hash_validator(hash)
        } else {
            false
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::block::is_valid_block_file;
    use std::{fs::File, io::Write};
    use tempfile::TempDir;

    #[test]
    fn test_nanomina_to_mina_conversion() {
        let actual = 1_000_000_001;
        let val = nanomina_to_mina(actual);
        assert_eq!("1.000000001", val);

        let actual = 1_000_000_000;
        let val = nanomina_to_mina(actual);
        assert_eq!("1", val);
    }

    #[test]
    fn test_pretty_print_duration() {
        assert_eq!(pretty_print_duration(Duration::from_secs(0)), "0s");
        assert_eq!(pretty_print_duration(Duration::from_secs(1)), "1s");
        assert_eq!(pretty_print_duration(Duration::from_secs(60)), "1m");
        assert_eq!(pretty_print_duration(Duration::from_secs(3661)), "1h 1m 1s");
        assert_eq!(
            pretty_print_duration(Duration::from_secs(86400 + 3661)),
            "1d 1h 1m 1s"
        );
        assert_eq!(
            pretty_print_duration(Duration::from_secs(172800 + 7200 + 120 + 5)),
            "2d 2h 2m 5s"
        );
    }

    fn create_temp_file(dir: &TempDir, name: &str, content: &str) -> PathBuf {
        let file_path = dir.path().join(name);
        let mut file = File::create(&file_path).unwrap();
        file.write_all(content.as_bytes()).unwrap();
        file_path
    }

    #[test]
    fn test_empty_vector() {
        let paths: Vec<PathBuf> = vec![];
        assert_eq!(calculate_total_size(&paths), 0);
    }

    #[test]
    fn test_single_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = create_temp_file(&temp_dir, "test1.txt", "Hello, World!");
        let paths = vec![file_path];
        assert_eq!(calculate_total_size(&paths), 13); // "Hello, World!" is 13
                                                      // bytes
    }

    #[test]
    fn test_multiple_files() {
        let temp_dir = TempDir::new().unwrap();
        let file1 = create_temp_file(&temp_dir, "test1.txt", "Hello");
        let file2 = create_temp_file(&temp_dir, "test2.txt", "World");
        let file3 = create_temp_file(&temp_dir, "test3.txt", "Rust");
        let paths = vec![file1, file2, file3];
        assert_eq!(calculate_total_size(&paths), 14); // "Hello" + "World" +
                                                      // "Rust" = 5 + 5 + 4 = 14
                                                      // bytes
    }

    #[test]
    fn test_nonexistent_file() {
        let temp_dir = TempDir::new().unwrap();
        let existing_file = create_temp_file(&temp_dir, "existing.txt", "I exist");
        let nonexistent_file = temp_dir.path().join("nonexistent.txt");
        let paths = vec![existing_file, nonexistent_file];
        assert_eq!(calculate_total_size(&paths), 7); // Only counts "I exist" (7
                                                     // bytes)
    }

    #[test]
    fn test_empty_file() {
        let temp_dir = TempDir::new().unwrap();
        let empty_file = create_temp_file(&temp_dir, "empty.txt", "");
        let paths = vec![empty_file];
        assert_eq!(calculate_total_size(&paths), 0);
    }

    #[test]
    fn test_is_valid_file_name() {
        // Valid cases
        assert!(is_valid_block_file(
            "mainnet-42-3Nabcdef12345678901234567890123456789012345678901234.json"
        ));

        assert!(!is_valid_block_file(
            "mainnet-3Nabcdef12345678901234567890123456789012345678901234.json"
        ));

        ///////////////////
        // Invalid cases //
        ///////////////////

        // Invalid hash (does not start with 3N)
        assert!(!is_valid_block_file(
            "mainnet-42-abcdef1234567890123456789012345678901234567890123456.json"
        ));

        // Hash too short
        assert!(!is_valid_block_file("mainnet-42-3Nabcdef1234.json"));

        // Missing hash part
        assert!(!is_valid_block_file("mainnet-42.json"));

        // Invalid extension
        assert!(!is_valid_block_file(
            "mainnet-42-3Nabcdef12345678901234567890123456789012345678901234.txt"
        ));

        // Too many parts
        assert!(!is_valid_block_file(
            "mainnet-42-3Nabcdef12345678901234567890123456789012345678901234-123.json"
        ));
    }
}
