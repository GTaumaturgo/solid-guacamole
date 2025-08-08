use std::collections::HashMap;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufReader, Read};

// Represents a move found in the opening book
#[derive(Clone, Debug)]
pub struct BookMove {
    pub move_uci: String, // The move in UCI format (e.g., "e2e4")
    pub weight: u16,      // Weight/probability of the move
}

// Represents an error that can occur while loading the book
#[derive(Debug)]
pub enum BookLoadError {
    IoError(io::Error),
    ParseError(String),
    NotPolyglotFile,
    UnsupportedFormat,
}

impl From<io::Error> for BookLoadError {
    fn from(err: io::Error) -> BookLoadError {
        BookLoadError::IoError(err)
    }
}

// The main data structure for the opening book
pub type OpeningBook = HashMap<u64, Vec<BookMove>>;

// Placeholder for Polyglot parsing logic
// This will need to be implemented or replaced with a crate.
mod polyglot_parser {
    use super::{BookMove, BookLoadError, OpeningBook};
    use std::io::Read;

    // Reference for Polyglot format:
    // http://hgm.nubati.net/book_format.html
    // Each entry is 16 bytes:
    // - key: u64 (Zobrist key)
    // - move: u16 (encoded move)
    // - weight: u16
    // - learn: u32 (unused by most engines)

    const ENTRY_SIZE: usize = 16;

    // Decodes a Polyglot move entry into a BookMove
    // This is a simplified placeholder and needs actual implementation
    // for converting Polyglot's u16 move format to UCI.
    fn decode_move_entry(key: u64, move_data: u16, weight: u16) -> Option<BookMove> {
        // TODO: Implement actual Polyglot move decoding
        // Polyglot move encoding:
        // fffrrrCFrrR (15 bits used)
        // fff: from file (a=0..h=7)
        // rrr: from rank (1=0..8=7)
        // C: promotion piece (0=N, 1=B, 2=R, 3=Q) - only if pawn promotion
        // F: to file
        // rr: to rank
        // R: special move flag (castling, en-passant, promotion) - this is more complex.
        // For now, we'll return a dummy string.
        // This needs a robust implementation based on the Polyglot spec.

        // A very naive placeholder. THIS IS NOT A REAL DECODER.
        let from_sq = (move_data >> 6) & 0x3F; // bits 6-11 for "from square" (0-63)
        let to_sq = move_data & 0x3F;          // bits 0-5 for "to square" (0-63)

        if from_sq >= 64 || to_sq >= 64 {
            return None; // Invalid square
        }

        let from_file = (from_sq % 8) as u8 + b'a';
        let from_rank = (from_sq / 8) as u8 + b'1';
        let to_file = (to_sq % 8) as u8 + b'a';
        let to_rank = (to_sq / 8) as u8 + b'1';

        // This doesn't handle promotions or special moves at all.
        let move_uci = format!(
            "{}{}{}{}",
            from_file as char, from_rank as char, to_file as char, to_rank as char
        );

        Some(BookMove { move_uci, weight })
    }


    pub fn parse_polyglot<R: Read>(mut reader: R) -> Result<OpeningBook, BookLoadError> {
        let mut book = OpeningBook::new();
        let mut buffer = [0u8; ENTRY_SIZE];

        loop {
            match reader.read_exact(&mut buffer) {
                Ok(_) => {
                    let key = u64::from_be_bytes(buffer[0..8].try_into().unwrap());
                    let move_data = u16::from_be_bytes(buffer[8..10].try_into().unwrap());
                    let weight = u16::from_be_bytes(buffer[10..12].try_into().unwrap());
                    // let _learn = u32::from_be_bytes(buffer[12..16].try_into().unwrap()); // learn data usually ignored

                    // The Polyglot spec has a "best move" convention for repeated keys,
                    // but most engines store all moves. We will store all moves.
                    if let Some(book_move) = decode_move_entry(key, move_data, weight) {
                        book.entry(key).or_insert_with(Vec::new).push(book_move);
                    } else {
                        // Potentially log a warning for undecodable moves
                        eprintln!("Warning: Could not decode move data {:#06x} for key {:#018x}", move_data, key);
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                    // End of file is expected
                    break;
                }
                Err(e) => {
                    return Err(BookLoadError::IoError(e));
                }
            }
        }

        if book.is_empty() && reader.read(&mut [0u8; 1])? > 0 { // Check if file was actually empty or just unparseable
             // This check is a bit naive, if the file is smaller than ENTRY_SIZE it will also trigger EOF earlier.
            return Err(BookLoadError::ParseError("No valid Polyglot entries found.".to_string()));
        }

        Ok(book)
    }
}

// Loads an opening book from the given file path.
// Currently attempts to load as a Polyglot .bin file.
pub fn load_book(file_path_str: &str) -> Result<OpeningBook, BookLoadError> {
    let file_path = Path::new(file_path_str);
    if !file_path.exists() {
        return Err(BookLoadError::IoError(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Book file not found: {}", file_path_str),
        )));
    }

    // Basic check for .bin extension, though not a guarantee of format.
    // A more robust check might involve reading magic numbers if the format specified them.
    // Polyglot doesn't have a magic number header.
    if file_path.extension().map_or(true, |ext| ext != "bin") {
         // For now, we are quite strict. Could be relaxed to try parsing anyway.
        // return Err(BookLoadError::UnsupportedFormat);
        // Let's actually try to parse it and let the parser fail if it's not polyglot.
        // This allows for files without .bin extension.
    }

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    polyglot_parser::parse_polyglot(reader)
}

// Function to get a move from the book for a given Zobrist key
pub fn get_book_move(book: &OpeningBook, position_key: u64) -> Option<BookMove> {
    book.get(&position_key).and_then(|moves| {
        if moves.is_empty() {
            None
        } else {
            // Simple strategy: pick the first move.
            // TODO: Implement a more sophisticated strategy (random, weighted random).
            moves.first().cloned()
            // For random selection:
            // use rand::seq::SliceRandom;
            // moves.choose(&mut rand::thread_rng()).cloned()
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to create a dummy polyglot book file for testing
    // This is a very basic version and does not correctly encode moves yet.
    fn create_dummy_bin_file(path: &str, entries: &[(u64, u16, u16)]) -> io::Result<()> {
        let mut file = File::create(path)?;
        for (key, r#move, weight) in entries {
            file.write_all(&key.to_be_bytes())?;
            file.write_all(&r#move.to_be_bytes())?;
            file.write_all(&weight.to_be_bytes())?;
            file.write_all(&0u32.to_be_bytes())?; // Learn data
        }
        Ok(())
    }

    #[test]
    fn test_load_simple_book() {
        let test_file = "test_book.bin";
        // (key, move_data, weight) - move_data is simplified for now
        // Let's use a "valid" looking from/to square for the naive decoder: e2e4
        // e2 = 1*8 + 4 = 12
        // e4 = 3*8 + 4 = 28
        // move_data for e2e4 (naive): (12 << 6) | 28 = 768 | 28 = 796
        let e2e4_naive_move_data = ((1 * 8 + 4) << 6) | (3 * 8 + 4); // e2e4

        let entries = [
            (12345_u64, e2e4_naive_move_data as u16, 10_u16), // Key 1, Move "e2e4" (naively), Weight 10
        ];
        create_dummy_bin_file(test_file, &entries).unwrap();

        let book = load_book(test_file).expect("Failed to load test book");
        assert_eq!(book.len(), 1);
        assert!(book.contains_key(&12345_u64));

        let moves = book.get(&12345_u64).unwrap();
        assert_eq!(moves.len(), 1);
        assert_eq!(moves[0].move_uci, "e2e4"); // This relies on the naive decoder
        assert_eq!(moves[0].weight, 10);

        std::fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_get_book_move() {
        let mut book = OpeningBook::new();
        let key1 = 12345_u64;
        let move1 = BookMove { move_uci: "e2e4".to_string(), weight: 10 };
        book.insert(key1, vec![move1.clone()]);

        let found_move = get_book_move(&book, key1).unwrap();
        assert_eq!(found_move.move_uci, "e2e4");

        let key2 = 67890_u64;
        assert!(get_book_move(&book, key2).is_none());
    }

    #[test]
    fn test_load_non_existent_book() {
        match load_book("non_existent_book.bin") {
            Err(BookLoadError::IoError(e)) if e.kind() == io::ErrorKind::NotFound => (), // Expected
            _ => panic!("Expected NotFound error"),
        }
    }

     #[test]
    fn test_load_empty_book_file() {
        let test_file = "empty_book.bin";
        File::create(test_file).unwrap().write_all(&[]).unwrap();
        let book = load_book(test_file);
        // Depending on how robust the parser is, this might be an empty book or a parse error.
        // The current polyglot_parser would return Ok(empty_book) if file is empty.
        // If file is < 16 bytes but not 0, it would be UnexpectedEof, then IoError.
        assert!(book.is_ok() && book.unwrap().is_empty(), "Empty file should result in an empty book or a specific error");
        std::fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_load_invalid_polyglot_file_too_small() {
        let test_file = "invalid_book_small.bin";
        File::create(test_file).unwrap().write_all(&[0u8; 10]).unwrap(); // Smaller than one entry
        match load_book(test_file) {
            Err(BookLoadError::IoError(e)) if e.kind() == io::ErrorKind::UnexpectedEof => (), // Expected
            Err(e) => panic!("Expected UnexpectedEof, got {:?}", e),
            Ok(_) => panic!("Expected error for too small file"),
        }
        std::fs::remove_file(test_file).unwrap();
    }
}
