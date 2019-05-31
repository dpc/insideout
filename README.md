<p align="center">
  <a href="https://travis-ci.org/dpc/insideout">
      <img src="https://img.shields.io/travis/dpc/insideout/master.svg" alt="Travis CI Build Status">
  </a>

  <a href="https://crates.io/crates/insideout">
      <img src="https://img.shields.io/crates/d/insideout.svg" alt="dpc on crates.io">
  </a>

  <a href="https://gitter.im/dpc/dpc">
      <img src="https://img.shields.io/gitter/room/dpc/dpc.svg" alt="dpc Gitter Chat">
  </a>
</p>

# insideout - Wrap composed types inside-out

Update: As of 1.33 a functionality like this is available in stdlib (at least for simple type). For iterators you can use `iterator.map(Result::transpose)` or `iterator.map(Option::transpose)` ([thread](https://internals.rust-lang.org/t/transpose-for-iterators/10321/2).

Turn `Option<Result<O, E>>` into `<Result<Option<O>, E>>` and the other way around.
Also for `Iterator`s and potentially other types.

Feel free to submit PR for other types where this operation makes sense.

