use crate::movegen::Move;

struct Transposition {
    hash: u64,
    depth: i32,
    score: i32,
    kind: TranspositionKind,
    _best_move: Move,
}

pub enum TranspositionKind {
    Exact,
    Alpha,
    Beta,
}

pub struct TranspositionTable {
    table: Box<[Transposition]>,

    pub stored_cnt: usize,
}

impl TranspositionTable {
    pub fn size(&self) -> usize {
        self.table.len()
    }

    pub fn new(entries: usize) -> Self {
        let table = (0..entries)
            .map(|_| Transposition {
                hash: 0,
                depth: 0,
                score: 0,
                kind: TranspositionKind::Exact,
                _best_move: Move::empty(),
            })
            .collect();

        Self {
            table,
            stored_cnt: 0,
        }
    }

    pub fn probe(&self, hash: u64, depth: i32, alpha: i32, beta: i32) -> Option<i32> {
        let idx = hash as usize % self.table.len();
        let entry = &self.table[idx];

        if entry.hash == hash && entry.depth >= depth {
            match entry.kind {
                TranspositionKind::Exact => Some(entry.score),
                TranspositionKind::Alpha => {
                    if entry.score <= alpha {
                        Some(entry.score)
                    } else {
                        None
                    }
                }
                TranspositionKind::Beta => {
                    if entry.score >= beta {
                        Some(entry.score)
                    } else {
                        None
                    }
                }
            }
        } else {
            None
        }
    }

    pub fn store(
        &mut self,
        hash: u64,
        depth: i32,
        score: i32,
        kind: TranspositionKind,
        best_move: Move,
    ) {
        self.stored_cnt += 1;
        let idx = hash as usize % self.table.len();

        self.table[idx] = Transposition {
            hash,
            depth,
            score,
            kind,
            _best_move: best_move,
        };
    }
}
