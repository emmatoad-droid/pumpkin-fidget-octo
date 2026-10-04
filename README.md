# pumpkin-fidget-octo

🎃 Pumpkin Fidget Octo 🐙

A tiny tactile crochet creature that accidentally became a Rust program.

This project started with yarn, a crochet hook, some pony beads, and absolutely no pre-existing pattern.

I made the Pumpkin Fidget Octo by hand, worked out how I'd constructed it, then wrote the pattern down. From there, the pattern went through several different forms:

crochet → written pattern → visual zine page → Rust → executable terminal output

So this repository is not an AI-generated crochet pattern.

The physical object came first.

The code is the computational version of something I had already made with my hands.

What is it?

A small pumpkin-shaped fidget octopus made from two crocheted circles, with:

* 🐙 8 tactile tentacle loops
* 🧡 4 pony beads per tentacle
* 🎃 chunky yarn
* 🔘 decorative buttons
* 🌿 optional green leaf/stalk
* 🧶 one continuous pre-threaded working yarn for the tentacles

Total beads:

8 × 4 = 32

The tentacles are designed to give you something tactile to fiddle with, slide, pull and move.

## The Rust version

The Rust program models the physical construction as data and behaviour.

It includes:

`enum YarnColor`

to represent yarn colours,

`enum Stitch`

to represent different crochet operations,

and:

`struct Octopus`

to hold the physical parameters of the object.

The program can calculate things such as:

* number of beads
* number of tentacles
* stitches per circle
* edge construction
* repeated tentacle operations

It also contains an edge trace simulator which walks through the construction step by step.

The current execution produces a 62-step trace of the seam and tentacle-building process.

## Why Rust?

Because apparently I looked at a crochet pattern and thought:

What if this was also a program?

😂

The interesting part wasn't turning crochet terminology into random code.

It was recognising that the physical object already contained structure:

objects → states → repeated operations → parameters → sequences

Once that structure was visible, it could be represented computationally.

The crochet pattern became a small domain model.

## The creative pipeline

This project moved through several different tools, but each stage had a different job.

```text
HANDS
  ↓
physical Pumpkin Fidget Octo
  ↓
observation
  ↓
written crochet pattern
  ↓
Gemini
  ↓
visual / zine-page interpretation
  ↓
DeepSeek
  ↓
Rust translation
  ↓
VS Code
  ↓
cargo run
  ↓
executable pattern trace
  ↓
GitHub
```

The important thing is that the original artefact came first.

The code is a translation of the thing, not the thing itself.

## Example output

Running:

```bash
cargo run
```

produces output including:

```text
🎃 PUMPKIN FIDGET OCTO 🎃
A quick, tactile desk beast for fidgety fingers.
PATTERN MATH
  Circle sts (each) ........ 20
  Edge join stitches ....... 22
  Tentacle loops ........... 8
  Beads per tentacle ....... 4
  TOTAL beads pre-threaded . 32
```

and then walks through the tentacle construction:

```text
[  8]     dc  anchor — tentacle 1
          ch  chain out — ch 10
[  9] bead-1  slide bead up + ch 1 to lock
[ 10] bead-2  slide bead up + ch 1 to lock
[ 11] bead-3  slide bead up + ch 1 to lock
[ 12] bead-4  slide bead up + ch 1 to lock
          ch  chain back — ch 10
[ 13] sl st  anchor back into dc
```

Eventually:

```text
── total counted steps: 62 ──
✨ zine page ready — print & fold ✨
```

## The clever bit

One of the physical construction decisions is pre-threading all the pony beads onto the working yarn before beginning the tentacles.

That means:

no snipping, no tying off 8 strands.

In the physical crochet object, that's simply a useful construction trick.

In the code, it became part of the model.

Which is exactly the sort of tiny detail that makes this project interesting to me: practical physical knowledge can become computational structure without losing the weirdness that made the original object worth making.

## Build

Create a normal Rust project and run:

```bash
cargo run
```

Or clone this repository and run it from the project directory.

Requires a working Rust toolchain with Cargo.

## Project status

Compiles. Runs. Prints the Pumpkin.

🎃 ✅
🐙 ✅
🧡 32 beads
🪢 8 tentacles
🦀 Rust
📄 zine-ready

## Why this exists

Because not everything needs to start as a software project.

Sometimes you make a stupid little crochet octopus.

Then you notice it has rules.

Then you write the rules down.

Then you wonder if the rules can run.

So you make them run.

And then, apparently, you put the whole fucking thing on GitHub.

⸻

Made by Emma — Chaos Gremlin Press / computational craft experiments
