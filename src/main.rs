use std::{
    io::Write,
    process::{Command, Stdio},
};

fn main() {
    let Some(path) = std::env::args().nth(1) else {
        println!("USAGE: dertect <path>");
        return;
    };

    let data = std::fs::read(path).unwrap();

    let candidates = {
        let mut candidates = Vec::new();

        for (offset, window) in data.windows(2).enumerate() {
            // ASN.1 SEQUENCE (0x30) followed by long length form with two length bytes (0x82).
            if window == [0x30, 0x82] {
                if let Some(length_slice) = data.get((offset + 2)..(offset + 2) + 2) {
                    let length = u16::from_be_bytes([length_slice[0], length_slice[1]]);
                    // 4 byte header followed by `length` data.
                    candidates.push(offset..(offset + 4 + length as usize));
                }
            }
        }

        candidates
    };

    for range in candidates {
        if let Some(data) = data.get(range) {
            let output = {
                let mut child = Command::new("der2ascii")
                    .stdin(Stdio::piped())
                    .spawn()
                    .unwrap();
                child.stdin.as_mut().unwrap().write_all(data).unwrap();
                child.wait_with_output().unwrap()
            };

            // Require successful der2ascii execution.
            if output.status.success() {
                // Require UTF-8 output.
                if let Ok(ascii) = String::from_utf8(output.stdout) {
                    // TODO: Ad-hoc filtering...
                    if ascii.contains("rsaEncryption") && !ascii.contains("commonName") {
                        println!("{}", ascii.trim());
                        println!("\n-------------------------\n");
                    }
                }
            }
        }
    }
}
