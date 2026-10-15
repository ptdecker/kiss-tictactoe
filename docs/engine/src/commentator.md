# The Commentator

The engine owns the words but not the randomness. It has no clock and no dependency on any platform,
so the front end supplies a `u64` seed when it creates a
[`Commentator`](../../../target/doc/kiss_tictactoe_engine/struct.Commentator.html). The desktop app
uses the system clock's nanoseconds and falls back to a fixed constant if the clock is unavailable.
A test harness can pass any constant.

## From seed to state

`Commentator::new(seed)` scrambles the seed with splitmix64, so seeds that differ only in their 
low bits (two clock readings a few nanoseconds apart) still give unrelated sequences. If the
scrambled value is zero, it is replaced with a nonzero constant, because zero is a fixed point of
the generator. The result is the state of a xorshift64 generator, a tiny shift-and-xor pseudo random
generator that needs no external crate.

## Choosing a line

Each event category has its own bank of four to eight lines, and a test keeps every bank in that 
range. Function `line(event)` draws a random index into the bank. If that category produced a line 
before, the previous pick is excluded, so the same line never appears twice in a row within a
category. Different categories track their last pick independently.

The banks themselves are private to the crate. Call `line` to get a `&'static str`.

## Determinism

The same seed gives the same sequence of lines for the same sequence of events. That makes a front
end's output reproducible in tests:

```rust
extern crate kiss_tictactoe_engine;
fn main() {
    use kiss_tictactoe_engine::{Commentator, TalkEvent};

    let events = [
        TalkEvent::GameStart,
        TalkEvent::GoodMove,
        TalkEvent::UserAboutToWin,
        TalkEvent::BadMove,
        TalkEvent::EngineWins,
    ];
    let mut a = Commentator::new(42);
    let mut b = Commentator::new(42);
    for event in events {
        assert_eq!(a.line(event), b.line(event));
    }
}
```

## Every event has words

```rust
extern crate kiss_tictactoe_engine;
fn main() {
    use kiss_tictactoe_engine::{Commentator, TalkEvent};

    let mut commentator = Commentator::new(1);
    for event in TalkEvent::ALL {
        assert!(!commentator.line(event).is_empty());
    }
}
```

## No immediate repeats

```rust
extern crate kiss_tictactoe_engine;
fn main() {
    use kiss_tictactoe_engine::{Commentator, TalkEvent};

    let mut commentator = Commentator::new(42);
    let mut previous = commentator.line(TalkEvent::GoodMove);
    for _ in 0..20 {
        let next = commentator.line(TalkEvent::GoodMove);
        assert_ne!(previous, next);
        previous = next;
    }
}
```

See the [event system](events.md) chapter for which event each turn produces, and
[Writing a new front end](new-front-end.md) for putting the two together.
