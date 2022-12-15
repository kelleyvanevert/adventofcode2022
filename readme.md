# Advent of Code 2022, using Rust

See https://adventofcode.com/2022

## Diary

- **Day 1.** Ah, it's not that hard! :)

- **Day 2.** First went all `enum Shape` and `impl FromStr` and `shape.score()` etc., but then later figured that for a script, simple `const`s make for way simpler and readable code :)

  - Learned: don't always need `enum/impl`

- **Day 3.** Note the funny `'a'..'{'` — I later learned the syntax `'a'..='z'` from Auke's solution :P

  - Learned:
    - `..=`
    - `code - ('a' as i32)`

- **Day 4.** I decided to go with an actual separate parser, figuring parsing the input of subsequent days would probably get harder and harder. A bit unnecessary here ofc. Also, how did I not know about `fs::read_to_string` in previous days? :P

  - Learned:
    - `fs::read_to_string`

- **Day 5.** The first significantly harder day! Not because of the parsing though (regexes work just fine), but because of data & ownership stuff I guess.

- **Day 6.** Surprisingly simple, too bad! :P

- **Day 7.** Harder again.

  - First I had an `enum Item { Folder, File }`, but then I had to write so many `match` blocks, even if using common data (like `name`), that I went for a more pragmatic `struct Node` with some redundancy. And then it was just a bit finnicky to get it all right.
  - Went immediately for a flat data structure w/ userland indirection instead of a recursive data structure, because I know that ownership gets really complicated otherwise — probably saved me quite some time.
  - For simplicity, I made `compute_folder_size` accumulate the folder sizes through a `&mut Vec<usize>` instead of trying to merge resulting vectors. => Q: is this the Rust way?

  - Learned:
    - `split_once`

- **Day 8.**

  - Wasn't too hard, but I did get into some finnicky trouble with `take_while` trying to get the bonus to work. The way the puzzle is written, it's more like `take_until(_and_including)`, which wasn't exactly what the crate `take-until` gives. And then I made a small attempt at creating my own `TakeUntil` trait + impl, before settling for quicker approach of just writing a very specific `fn count_until`.
  - Also the performance won't be optimal, but the immediate fix is to decide in which order to check the lines of vision for visibility (instead of always checking up, down, left, right).
  - Learned:
    - Didn't actually learn today, but was reminded again of how taking a `mut p` where `P: FnMut` is more general than just `p: fn`. Not actually necessary even, but, I just took it from the `TakeWhile` implementation, and now `count_until` could also choose to be a closure that mutates.

- **Day 9.** Fun, and especially a fun generalization in the bonus exercise this time :)

- **Day 10.** Not really sure how the ownership transfers work, between the mutating closure and it's surrounding scope in `solve`, but, it works :D

- **Day 11.** That was quite a bit harder! For three main reasons:

  1. The parsing. I wanted to do it a bit more "defensively", and so I turned to the builder pattern for constructing the `Monkey` instances. After starting writing it myself, I decided a crate must exist for it, and I started using `derive_builder`. Then I spend some time fiddling with the level of "defensiveness" in the line parsing loop, struggling with putting the builder in an `Option` but also keeping it mutable, which turned out to be a hassle. In the end I just kept is pragmatic and simple.
     - I like my cute little `MiniExpr` / `MiniNode` structs with their `parse` and `eval` implementations :P
  2. Fighting with the borrow checker over mutating other monkeys while looping over the monkeys in each round. There must be a more elegant way of doing this, but, I was already spending so much time on today's challenge, that I just went with whatever worked :P
  3. The bonus. I first tried the "silly thing" and just rewrote it to use `BigInt`s, but, that wouldn't have worked anyway, because the numbers get unmanageably large and the code would not stop running. The trick is that we're working modulo `n`, where I first (incorrectly) set `n` to each respective monkey's divisibility check number, but then later, after taking a break to help my roommate buy stuff for Christmas dinner, figured that it should be all these monkeys' numbers multiplied by each other, because the items move hands and therefore shouldn't lose information in between.

  TODO:

  - Learn better error handling, i.e. no more `type Err = ();`
  - Learn how to do the mutable iterator & vector updates thing (point 2) in idiomatic Rust

- **Day 12.** Ai! We seem to have gotten to the part of the advent calendar where every challenge is getting hard :P Where yesterday, for the bonus star, you needed a bit of mathematical knowledge, today, you need a programmer's trick: Instead of just performing the entire search algorithm once for every possible starting point, which amounts to a very lengthy computation, it's better to generalize the algorithm, which then interestingly takes less time also :P What a fun bonus star generalization again!

- **Day 13.** Today had three main parts, for me:

  - Getting acquainted with Nom
  - First failing at `PartialOrd for Packet`, and then reimplementing it after
  - ..getting the actual ordering computation right -- trickier than I thought

  Learned:

  - Nom
  - `<iter>.position(<predicate>)`
  - `PartialOrd`, `Ord`
  - Putting `use crate::{whatever}` imports inside the test function keeps the linter happy :)

- **Day 14.** The bad news is that it's taking me longer every day to solve. The good news is that I'm still having a lot of fun doing so :)

- **Day 15.** Wow, ok, this one really does need smart thinking. I first implemented it the naive way, but quickly found out that that won't do. Then I implemented a `Spans` type that keeps track of a list of spans, that it keeps sorted and as small as possible, so that I don't need for-loops to check the grid but can just work with equations. That's a perfectly reasonable solution for the first part, but it doesn't generalize automatically to the second part. It did allow me to "brute-force" the second part though, which takes about 11 seconds. After that, I went to Reddit for the first time, to check what other people did, and it indeed turns out that you have to get quite creative to solve today's challenge in a performant way. Lots of mathematical and programmatic tricks that I would never have thought of! It seems I'm gonna have to up my game for the second half of the calendar! :P
