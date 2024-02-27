pub struct Piece {
    start: usize,
    length: usize,
    added: bool,
}

pub struct PieceTable {
    original: String,
    add: String,
    pieces: Vec<Piece>,
}

impl PieceTable {
    pub fn new(string: &str) -> Self {
        Self {
            original: String::from(string.to_string()),
            add: String::new(),
            pieces: vec![Piece {
                start: 0,
                length: string.len(),
                added: false,
            }],
        }
    }

    pub fn insert(&mut self, pos: usize, text: &str) -> &mut Self {
        self.replace(pos, pos, text)
    }

    pub fn delete(&mut self, pos: usize, length: usize) -> &mut Self {
        let end = pos + length;
        self.replace(pos, end, "")
    }

    fn replace(&mut self, start: usize, end: usize, text: &str) -> &mut Self {
        // add_buffer_start = length of the 'add' buffer before adding 'text'
        let add_buffer_start = self.add.len();
        // Add text to the add buffer
        self.add.push_str(text);
        
        
        // find the pieces that need to be replaced:
        //     for each piece in pieces:
        // Keep track of the offset from the start of the text
        let mut offset = 0;
        let mut piece_index: Option<usize> = None;
        for (i, piece) in self.pieces.iter_mut().enumerate() {
            if start < (offset + piece.length) {
                if start == offset {
                    // If the start position is at the beginning of a piece, we don't need to split the piece
                    piece_index = Some(i);
                    break;
                }
                // Create a new piece representing the text after the start position
                let new_piece = Piece {
                    start: piece.start + (start - offset), // Start from the replacement position
                    length: piece.length - (start - offset), // The remaining length after the split
                    added: piece.added, // Keep the same 'added' status as the original piece
                };
                let split_length = start - offset;
                piece.length = split_length; // Truncate the current piece at the start position
                self.pieces.insert(i + 1, new_piece); // Insert the new piece after the current piece
                piece_index = Some(i + 1);
                offset += split_length;
                // We've found the piece that needs to be replaced, so we can break out of the loop.
                break;
            }
            offset += piece.length;
        }
        

        if let Some(index) = piece_index {
            let mut pieces_to_remove = 0;
            for removed_piece in &self.pieces[index..] {
                let end_offset = removed_piece.start + removed_piece.length;
                if offset >= start && (end_offset) <= end {
                    // Increment the count of pieces to be removed
                    pieces_to_remove += 1;
                    // Move the offset to the end of the piece we're removing
                    offset += removed_piece.length;
                } else {
                    break;  // Exit the loop if we reach a piece beyond the end position
                }
            }
    
            // Remove the pieces fully contained within the [start, end) range
            if pieces_to_remove > 0 {
                self.pieces.drain(index..index + pieces_to_remove);
            }
            // Check if the end of the text to be replaced splits a piece
            if offset < end {
                // Get a mutable reference to the piece that is split by the end position
                let last_piece = &mut self.pieces[index - pieces_to_remove];

                let change = end - offset;
                last_piece.start = last_piece.start + change;
                last_piece.length = last_piece.length - change;
            }

            if text.len() > 0 {
                self.pieces.insert(index, Piece {
                    start: add_buffer_start,
                    length: text.len(),
                    added: true,
                });
            }
        } else {
            if text.len() > 0 {
                self.pieces.push(Piece {
                    start: add_buffer_start,
                    length: text.len(),
                    added: true,
                });
            }
        }
        self
    }

    pub fn content(&self) -> String {
        let mut content = String::new();
        for piece in &self.pieces {
            let text = if piece.added {
                &self.add[piece.start..piece.start + piece.length]
            } else {
                &self.original[piece.start..piece.start + piece.length]
            };
            content.push_str(text);
        }
        content
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_piece_table_end_insertion() {
        let mut piece_table = PieceTable::new("Hello World");
        piece_table.insert(11, "!");
        assert_eq!(piece_table.content(), "Hello World!");
    }

    #[test]
    fn test_piece_table_insertion() {
        let mut piece_table = PieceTable::new("Hello World");
        piece_table.insert(6, "Beautiful ");
        assert_eq!(piece_table.content(), "Hello Beautiful World");
    }    
    
    #[test]
    fn test_piece_table_deletion() {
        let mut piece_table = PieceTable::new("Hello Beautiful World");
        piece_table.delete(6, 10);
        assert_eq!(piece_table.content(), "Hello World");
    }

    #[test]
    fn test_piece_table_multiple_operations() {
        let mut piece_table = PieceTable::new("Hello World");
        piece_table.insert(5, ",");
        assert_eq!(piece_table.content(), "Hello, World");
        piece_table.insert(6, " Beautiful");
        assert_eq!(piece_table.content(), "Hello, Beautiful World");
        piece_table.delete(5, 1);
        assert_eq!(piece_table.content(), "Hello Beautiful World");
    }
}
