use std::collections::HashMap;

use bit_iter::BitIter;
use slotmap::{DefaultKey, SlotMap};

use crate::parser::{Board, Boards, LineOfNumbers};

#[derive(Clone, Copy)]
pub enum Span {
    Row(u128),
    Col(u128),
    Board(usize, usize),
}

impl std::fmt::Debug for Span {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Row(arg0) => {
                let mut d = f.debug_tuple("Row");
                for b in BitIter::from(*arg0) {
                    d.field(&b);
                }
                d.finish()
            }
            Self::Col(arg0) => {
                let mut d = f.debug_tuple("Col");
                for b in BitIter::from(*arg0) {
                    d.field(&b);
                }
                d.finish()
            }
            Self::Board(arg0, arg1) => f.debug_tuple("Board").field(arg0).field(arg1).finish(),
        }
    }
}

impl Span {
    fn insert(&mut self, num: u8) {
        match self {
            Span::Row(x) => *x |= 1 << num,
            Span::Col(x) => *x |= 1 << num,
            Span::Board(_, x) => *x += num as usize,
        }
    }

    pub fn score(&self) -> Option<(usize, usize)> {
        match self {
            Span::Board(i, x) => Some((*i, *x)),
            _ => None,
        }
    }

    pub fn is_complete(&self) -> bool {
        match self {
            Span::Col(x) | Span::Row(x) => x.count_ones() == 0,
            _ => false,
        }
    }

    pub fn call_number(&mut self, num: u8) -> bool {
        match self {
            Span::Row(x) => *x &= !(1 << num),
            Span::Col(x) => *x &= !(1 << num),
            Span::Board(_, x) => *x -= num as usize,
        }
        self.is_complete()
    }
}

#[derive(Debug, Clone)]
pub struct Entry {
    row: DefaultKey,
    col: DefaultKey,
    board: DefaultKey,
}

#[derive(Debug, Default)]
pub struct Index {
    cleanup_spans: HashMap<usize, Vec<DefaultKey>>,
    cleanup_entries: HashMap<usize, Vec<DefaultKey>>,
    spans: SlotMap<DefaultKey, Span>,
    entries: SlotMap<DefaultKey, Entry>,
    numbers: HashMap<u8, Vec<DefaultKey>>,
}

impl Index {
    pub fn call_number(&mut self, num: u8) -> Option<Vec<(usize, usize)>> {
        if let Some(winners) = self.inner_call_number(num) {
            for (i, _) in winners.iter() {
                if let Some(spans) = self.cleanup_spans.remove(i) {
                    for s in spans.into_iter() {
                        self.spans.remove(s);
                    }
                }

                if let Some(entries) = self.cleanup_entries.remove(i) {
                    for e in entries.into_iter() {
                        self.entries.remove(e);
                    }
                }
            }

            return Some(winners);
        }

        None
    }

    fn inner_call_number(&mut self, num: u8) -> Option<Vec<(usize, usize)>> {
        let mut winners = Vec::new();
        if let Some(entries) = self.numbers.get(&num) {
            for entry_key in entries.iter() {
                if let Some([row, col, board]) = self
                    .entries
                    .get(*entry_key)
                    .and_then(|e| self.spans.get_disjoint_mut([e.row, e.col, e.board]))
                {
                    board.call_number(num);
                    let row_done = row.call_number(num);
                    let col_done = col.call_number(num);
                    if row_done || col_done {
                        if let Some(winner) = board.score().map(|(i, s)| (i, s * num as usize)) {
                            winners.push(winner);
                        }
                    }
                }
            }
        }

        (!winners.is_empty()).then(|| winners)
    }

    fn insert_board(&mut self, i: usize, board: Board) {
        let mut board_span = Span::Board(i, 0);
        let mut rows = [Span::Row(0); 5];
        let mut cols = [Span::Col(0); 5];

        Self::populate_spans(&mut board_span, &mut rows, &mut cols, &board);

        let mut row_keys = [DefaultKey::default(); 5];
        let mut col_keys = [DefaultKey::default(); 5];
        let board_key = self.insert_span(i, board_span);

        self.insert_spans(i, &mut row_keys, &rows);
        self.insert_spans(i, &mut col_keys, &cols);
        self.insert_entries(i, board_key, &row_keys, &col_keys, &board);
    }

    fn populate_spans(
        board_span: &mut Span,
        rows: &mut [Span],
        cols: &mut [Span],
        board: &[LineOfNumbers],
    ) {
        for (row, board_row) in rows.iter_mut().zip(board.iter()) {
            for (col, num) in cols.iter_mut().zip(board_row.iter()) {
                row.insert(*num);
                col.insert(*num);
                board_span.insert(*num);
            }
        }
    }

    fn insert_spans(&mut self, i: usize, keys: &mut [DefaultKey], spans: &[Span]) {
        for (span, slot) in spans.iter().zip(keys.iter_mut()) {
            *slot = self.insert_span(i, *span);
        }
    }

    fn insert_span(&mut self, i: usize, span: Span) -> DefaultKey {
        let key = self.spans.insert(span);
        self.cleanup_spans.entry(i).or_default().push(key);
        key
    }

    fn insert_entries(
        &mut self,
        i: usize,
        board_key: DefaultKey,
        rows: &[DefaultKey],
        cols: &[DefaultKey],
        board: &[LineOfNumbers],
    ) {
        let entry_cleanup_list = self.cleanup_entries.entry(i).or_default();
        for (row, board_row) in rows.iter().copied().zip(board.iter()) {
            for (col, num) in cols.iter().copied().zip(board_row.iter()) {
                let entry_key = self.entries.insert(Entry {
                    row,
                    col,
                    board: board_key,
                });

                entry_cleanup_list.push(entry_key);
                self.numbers.entry(*num).or_default().push(entry_key);
            }
        }
    }
}

impl From<Boards> for Index {
    fn from(boards: Boards) -> Self {
        let mut index = Self::default();

        for (i, board) in boards.into_iter().enumerate() {
            index.insert_board(i, board);
        }

        index
    }
}
