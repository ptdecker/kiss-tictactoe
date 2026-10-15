//! The engine's voice: message banks and a seeded, non-repeating selector.
//!
//! Randomness is a std-only xorshift64 generator with no external crates
//! (D-07).

use crate::types::TalkEvent;

const GAME_START: &[&str] = &[
    "Ready to lose? I'll go easy. Not really.",
    "Pick a square. Any square. It won't matter.",
    "Another challenger. How refreshing. And brief.",
    "Your move, champ. Try to look like you have a plan.",
    "Take your time. It won't help.",
];

const ENGINE_WINS: &[&str] = &[
    "Another one for the win column. Mine.",
    "Did you want a rematch, or just a nap?",
    "Predictable. Still enjoyable.",
    "That went about as expected.",
    "Better luck next time. Or the time after.",
];

// Unreachable against the perfect engine until TRAIN-01 (v2 training mode).
const USER_WINS: &[&str] = &[
    "Impossible. I demand a recount.",
    "Fine. You win. Enjoy it.",
    "A glitch. That was clearly a glitch.",
    "Well played. Do not get used to it.",
];

const DRAW: &[&str] = &[
    "A draw. The coward's victory.",
    "Nobody wins. Nobody learns.",
    "Stalemate. Try harder next time.",
    "Even. Barely. Not that I was trying.",
];

const ENGINE_ABOUT_TO_WIN: &[&str] = &[
    "Two in a row. Your move to lose.",
    "I smell victory. It smells like me.",
    "Block this. Or don't. I'm easy.",
    "Tick tock. The third square is mine.",
];

const USER_ABOUT_TO_WIN: &[&str] = &[
    "Cute. Two in a row. Watch me ruin it.",
    "Oh, a plan. How adorable. Blocked.",
    "You almost had something. Almost.",
    "Nice try. That square is mine now.",
];

const GOOD_MOVE: &[&str] = &[
    "Fine. That was not terrible.",
    "Careful. You might get the hang of this.",
    "A sensible move. I'm shocked.",
    "Hmm. Not a mistake. Yet.",
];

const BAD_MOVE: &[&str] = &[
    "Bold choice. Wrong, but bold.",
    "Was that on purpose? Please say no.",
    "I'll take it. Thanks for the gift.",
    "Oof. Even I felt that one.",
];

/// The message bank for one event category.
pub(crate) fn bank(event: TalkEvent) -> &'static [&'static str] {
    match event {
        TalkEvent::GameStart => GAME_START,
        TalkEvent::EngineWins => ENGINE_WINS,
        TalkEvent::UserWins => USER_WINS,
        TalkEvent::Draw => DRAW,
        TalkEvent::EngineAboutToWin => ENGINE_ABOUT_TO_WIN,
        TalkEvent::UserAboutToWin => USER_ABOUT_TO_WIN,
        TalkEvent::GoodMove => GOOD_MOVE,
        TalkEvent::BadMove => BAD_MOVE,
    }
}

/// One splitmix64 step: a cheap bijective scrambler for seed values.
fn splitmix64(seed: u64) -> u64 {
    let mut z = seed.wrapping_add(0x9E37_79B9_7F4A_7C15);
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
    z ^ (z >> 31)
}

/// Picks trash-talk lines for [`TalkEvent`]s using a seeded PRNG.
///
/// The same seed yields the same sequence, and a category never returns the
/// same line twice in a row.
///
/// # Example
///
/// ```
/// use kiss_tictactoe_engine::{Commentator, TalkEvent};
///
/// let mut c = Commentator::new(42);
/// let line = c.line(TalkEvent::GameStart);
/// assert!(!line.is_empty());
/// ```
#[derive(Debug, Clone)]
pub struct Commentator {
    state: u64,
    last: [Option<usize>; TalkEvent::COUNT],
}

impl Commentator {
    /// Create a commentator from `seed`.
    ///
    /// The seed is scrambled with splitmix64 first, so raw clock values that
    /// differ only in their low bits still give unrelated sequences. A mixed
    /// state of 0 is remapped to a nonzero constant, since xorshift has 0 as
    /// a fixed point.
    pub fn new(seed: u64) -> Self {
        let mixed = splitmix64(seed);
        Self {
            state: if mixed == 0 {
                0x9E37_79B9_7F4A_7C15
            } else {
                mixed
            },
            last: [None; TalkEvent::COUNT],
        }
    }

    fn next(&mut self) -> u64 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.state = x;
        x
    }

    /// Pick a line for `event`, never repeating the previous line returned
    /// for that same category.
    pub fn line(&mut self, event: TalkEvent) -> &'static str {
        let lines = bank(event);
        let n = lines.len() as u64;
        // The no-repeat pick draws from `n - 1` slots, so a bank needs at
        // least two lines (also guarded by `every_bank_has_4_to_8_lines`).
        debug_assert!(n >= 2, "bank for {event:?} needs at least 2 lines");
        let idx = match self.last[event as usize] {
            None => (self.next() % n) as usize,
            Some(prev) => {
                let mut i = (self.next() % (n - 1)) as usize;
                if i >= prev {
                    i += 1;
                }
                i
            }
        };
        self.last[event as usize] = Some(idx);
        lines[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn every_bank_has_4_to_8_lines() {
        for ev in TalkEvent::ALL {
            assert!((4..=8).contains(&bank(ev).len()), "{ev:?}");
        }
    }

    #[test]
    fn every_line_is_short_and_nonempty() {
        for ev in TalkEvent::ALL {
            for l in bank(ev) {
                assert!(!l.trim().is_empty());
                assert!(l.chars().count() <= 56, "too long: {l}");
            }
        }
    }

    #[test]
    fn banks_total_32_to_64() {
        let total: usize = TalkEvent::ALL.iter().map(|e| bank(*e).len()).sum();
        assert!((32..=64).contains(&total), "total {total}");
    }

    #[test]
    fn lines_unique_within_bank() {
        for ev in TalkEvent::ALL {
            let b = bank(ev);
            for (i, a) in b.iter().enumerate() {
                assert!(!b[i + 1..].contains(a), "duplicate: {a}");
            }
        }
    }

    #[test]
    fn same_seed_same_sequence() {
        let mut a = Commentator::new(123);
        let mut b = Commentator::new(123);
        for i in 0..100 {
            let ev = TalkEvent::ALL[i % TalkEvent::COUNT];
            assert_eq!(a.line(ev), b.line(ev));
        }
    }

    #[test]
    fn seed_zero_is_not_stuck() {
        let mut c = Commentator::new(0);
        let set: std::collections::HashSet<_> =
            (0..50).map(|_| c.line(TalkEvent::GameStart)).collect();
        assert!(set.len() >= 2);
    }

    #[test]
    fn adjacent_seeds_diverge() {
        let seq = |seed| {
            let mut c = Commentator::new(seed);
            (0..8)
                .map(|_| c.line(TalkEvent::GameStart))
                .collect::<Vec<_>>()
        };
        assert_ne!(seq(1), seq(2));
    }

    #[test]
    fn all_lines_reachable() {
        let mut c = Commentator::new(1);
        for ev in TalkEvent::ALL {
            let seen: std::collections::HashSet<_> = (0..500).map(|_| c.line(ev)).collect();
            assert_eq!(seen.len(), bank(ev).len(), "{ev:?}");
        }
    }

    proptest! {
        #[test]
        fn never_repeats_consecutively(seed in any::<u64>(), idx in 0usize..8) {
            let ev = TalkEvent::ALL[idx];
            let mut c = Commentator::new(seed);
            let mut prev = c.line(ev);
            for _ in 0..50 {
                let cur = c.line(ev);
                prop_assert_ne!(prev, cur);
                prev = cur;
            }
        }
    }
}
