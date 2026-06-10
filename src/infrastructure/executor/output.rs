// ─── < Imports > ────────────────────────────────────────────────────

use std::io::Read;
use std::thread;

// ─── < Structs > ────────────────────────────────────────────────────

#[derive(Default)]
pub(super) struct CapturedOutput {
    pub content: String,
    pub truncated: bool,
}

// ─── < Public Functions > ───────────────────────────────────────────

pub(super) fn capture_output<R>(reader: Option<R>, max_output_bytes: usize) -> thread::JoinHandle<CapturedOutput>
where
    R: Read + Send + 'static,
{
    thread::spawn(move || {
        let Some(reader) = reader else {
            return CapturedOutput::default();
        };

        read_capped_output(reader, max_output_bytes)
    })
}

pub(super) fn join_output(reader: thread::JoinHandle<CapturedOutput>) -> CapturedOutput {
    reader.join().unwrap_or_default()
}

// ─── < Private Functions > ──────────────────────────────────────────

fn read_capped_output(mut reader: impl Read, max_output_bytes: usize) -> CapturedOutput {
    let mut stored = Vec::new();
    let mut buffer = [0_u8; 8192];
    let mut truncated = false;

    loop {
        let bytes_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(bytes_read) => bytes_read,
            Err(_) => break,
        };

        if stored.len() < max_output_bytes {
            let remaining_capacity = max_output_bytes - stored.len();
            let bytes_to_store = bytes_read.min(remaining_capacity);

            stored.extend_from_slice(&buffer[..bytes_to_store]);

            if bytes_to_store < bytes_read {
                truncated = true;
            }
        } else {
            truncated = true;
        }
    }

    CapturedOutput {
        content: String::from_utf8_lossy(&stored).to_string(),
        truncated,
    }
}
