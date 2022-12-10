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
