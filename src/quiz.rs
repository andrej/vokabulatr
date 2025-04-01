pub struct Flashcard {
    pub front: String,
    pub back: String,
    pub statistics: FlashcardStatistics
}

pub struct FlashcardStatistics {
    pub attempts: u32,
    pub correct: u32,
    pub history: u64 // Stores a 1 bit for each correct attempt out of the last min(64, attempts) attempts
}

impl FlashcardStatistics {
    pub fn new() -> Self {
        Self {
            attempts: 0,
            correct: 0,
            history: 0
        }
    }

    /// Record an attempt
    pub fn record_attempt(&mut self, correct: bool) {
        self.attempts += 1;
        self.history <<= 1;
        if correct {
            self.correct += 1;
            self.history |= 1;
        }
    }

    /// Returns how many attempts the recent history encapsulates
    pub fn recent_attempts(&self) -> u32 {
        u32::min(64, self.attempts)
    }

    /// Returns how many out of the most recent min(64, attempts) attempts were
    /// correct
    pub fn recently_correct(&self) -> u32 {
        let mut count: u32 = 0;
        let mut history = self.history;
        while history > 0 {
            count += (history & 1) as u32;
            history >>= 1;
        }
        count
    }

    /// Format recent history into a string
    pub fn history_string(&self, correct_char: &str, incorrect_char: &str) -> String {
        let mut ret = String::new();
        let oldest = self.recent_attempts();
        ret.reserve(oldest as usize);
        for i in (0..oldest).rev() {
            if 1 == (self.history >> i) & 1 {
                ret += correct_char;
            } else {
                ret += incorrect_char;
            }
        }
        ret
    }
}

pub struct Quiz {
    pub items: Vec<Flashcard>,
    pub selection: Vec<usize>,
    pub current: usize,
    pub aggregate_statistics: FlashcardStatistics,
}

#[derive(serde::Deserialize)]
struct CSVFlashcard {
    front: String,
    back: String
}

pub struct Matcher {
    trim_whitespace: bool,
    normalize_whitespace: bool,
    ignore_case: bool,
    ignore_accents: bool,
    ignore_nonalphabetic: bool
}

fn normalize_whitespace(input: &str) -> String {
    let mut output = String::new();
    for word in input.split_whitespace() {
        output += word;
        output += " ";
    }
    output.trim().to_string()
}

fn remove_nonalphabetic(input: &str) -> String {
    input.chars().filter(|c| { c.is_alphabetic() }).collect()
}

impl Matcher {
    pub fn new() -> Self {
        Self {
            trim_whitespace: true,
            normalize_whitespace: true,
            ignore_case: true,
            ignore_accents: true,
            ignore_nonalphabetic: true
        }
    }
    pub fn matches(&self, a: &str, b: &str) -> bool {
        let mut a: String = a.to_string();
        let mut b: String = b.to_string();
        if self.trim_whitespace {
            a = a.trim().to_string();
            b = b.trim().to_string();
        }
        if self.normalize_whitespace {
            a = normalize_whitespace(a.as_str());
            b = normalize_whitespace(b.as_str());
        }
        if self.ignore_case {
            a = a.to_lowercase();
            b = b.to_lowercase();
        }
        if self.ignore_accents {
            a = deunicode::deunicode(a.as_str());
            b = deunicode::deunicode(b.as_str());
        }
        if self.ignore_nonalphabetic {
            a = remove_nonalphabetic(a.as_str());
            b = remove_nonalphabetic(b.as_str());
        }
        a == b
    }
}

impl Quiz {
    pub fn from_csv_input(input: impl std::io::Read) -> Self {
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .from_reader(input);
        let mut items = Vec::<Flashcard>::new();
        for maybe_record in reader.deserialize() {
            let record: CSVFlashcard = maybe_record.expect("invalid CSV record");
            items.push(Flashcard {
                front: record.front,
                back: record.back,
                statistics: FlashcardStatistics::new()
            });
        }
        let n_items = items.len();
        assert!(n_items > 0);
        let mut ret = Self {
            items,
            selection: Vec::<usize>::new(),
            current: n_items - 1,
            aggregate_statistics: FlashcardStatistics::new()
        };
        ret.select_all();
        ret
    }

    pub fn select_all(&mut self) {
        self.selection = (0..(self.items.len())).collect()
    }

    pub fn shuffle_selection(&mut self) {
        let mut rng = rand::rng();
        rand::prelude::SliceRandom::shuffle(&mut self.selection[..], &mut rng);
    }

    /// Refine the current selection to only include the hardest n words,
    /// measured by aboslute number of times these words were entered wrong
    pub fn select_hardest(&mut self, n: usize) {
        // Heap of tuples (wrong_attempts, index)
        let mut hardest = std::collections::BinaryHeap::<(u32, usize)>::new();
        for index in &self.selection {
            let flashcard = &self.items[*index];
            let wrong_attempts = flashcard.statistics.attempts - flashcard.statistics.correct;
            hardest.push((wrong_attempts, *index));
        }
        self.selection.clear();
        let mut i = 0;
        while i < n && hardest.len() > 0 {
            let (_, index) = hardest.pop().unwrap();
            self.selection.push(index);
            i += 1;
        }
    }

    pub fn current(&self) -> &Flashcard {
        assert!(self.items.len() > 0);
        assert!(self.selection.len() > 0);
        &self.items[self.selection[self.current]]
    }

    pub fn advance(&mut self) {
        assert!(self.selection.len() > 0);
        self.current = (self.current + 1) % self.items.len();
    }

    pub fn record_attempt(&mut self, correct: bool) {
        let stats = &mut self.items[self.current].statistics;
        stats.record_attempt(correct);
        self.aggregate_statistics.record_attempt(correct);
    }

}