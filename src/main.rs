//! 🎃 Pumpkin Fidget Octo — zine pattern generator + edge simulator
//! Build: cargo new pumpkin_octo && paste into src/main.rs
//! Run:   cargo run

use std::fmt;

// ---------- Types ----------

#[derive(Debug, Clone, Copy)]
enum YarnColor {
    Orange,
    Green,
}

impl fmt::Display for YarnColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            YarnColor::Orange => write!(f, "Orange"),
            YarnColor::Green => write!(f, "Green"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Stitch {
    Chain(u8),
    DoubleCrochet,
    SlipStitch,
}

impl Stitch {
    fn abbr(&self) -> &'static str {
        match self {
            Stitch::Chain(_) => "ch",
            Stitch::DoubleCrochet => "dc",
            Stitch::SlipStitch => "sl st",
        }
    }
}

// ---------- The Octopus ----------

struct Octopus {
    circles: u8,
    beads_per_tentacle: u8,
    tentacle_count: u8,
    dc_between_tentacles: u8,
}

impl Default for Octopus {
    fn default() -> Self {
        Self {
            circles: 2,
            beads_per_tentacle: 4,
            tentacle_count: 8,
            dc_between_tentacles: 6,
        }
    }
}

impl Octopus {
    fn total_beads(&self) -> u32 {
        self.beads_per_tentacle as u32 * self.tentacle_count as u32
    }

    fn circle_stitch_count(&self) -> u32 {
        20
    }

    /// 1 (join) + dc_between + tentacles + dc_between + 1 (closing join)
    fn edge_stitch_count(&self) -> u32 {
        1 + self.dc_between_tentacles as u32
            + self.tentacle_count as u32
            + self.dc_between_tentacles as u32
            + 1
    }

    // ---------- Pattern printing ----------

    fn print_header(&self) {
        println!("🎃 PUMPKIN FIDGET OCTO 🎃");
        println!("A quick, tactile desk beast for fidgety fingers.\n");
    }

    fn print_materials(&self) {
        println!("MATERIALS");
        println!("  • 6mm crochet hook");
        println!(
            "  • Light chunky weight yarn ({} & {} scrap)",
            YarnColor::Orange,
            YarnColor::Green
        );
        println!("  • ~{} pony beads", self.total_beads());
        println!("  • Decorative button (star or round)");
        println!("  • Darning needle & safety pins / stitch markers\n");
    }

    fn print_body(&self) {
        println!("THE BODY (Make {})", self.circles);
        println!("  Round 1: ML, ch 1. 10 tr into loop. sl st into first tr. (10 sts)");
        println!("  Round 2: ch 1. 2 tr into same st. 2 tr into each st around.");
        println!(
            "           sl st, fasten off. ({} sts)",
            self.circle_stitch_count()
        );
        println!("  Repeat Rounds 1 & 2 for the second circle.\n");
    }

    fn print_prep(&self) {
        println!("BUTTONS & PREP");
        println!("  1. Pull the magic-loop tail up through the center; stitch button to RS.");
        println!("  2. Repeat for circle 2 (or weave in tail).");
        println!("  3. Pin both circles together, right sides facing outward.");
        println!(
            "  4. Pre-thread ~{} pony beads onto your working ball of yarn.",
            self.total_beads()
        );
        println!("     (This is the clever bit — no snipping, no tying off 8 strands.)\n");
    }

    fn print_tentacles(&self) {
        println!("TENTACLES & ASSEMBLY");
        println!(
            "  1. Join yarn at top through both layers. ch 1, then dc through both circles."
        );
        println!(
            "  2. dc through the next {} sts around the edge.",
            self.dc_between_tentacles
        );
        println!(
            "  3. On Stitch {}: dc through both layers.",
            self.dc_between_tentacles + 1
        );
        println!("       • ch 10");
        println!("       • for each bead: slide 1 up working yarn, ch 1 around it to lock");
        println!("         ({} beads per strand)", self.beads_per_tentacle);
        println!("       • ch 10, sl st back into the dc anchor");
        println!(
            "  4. Repeat Step 3 {} times across the bottom edge.",
            self.tentacle_count
        );
        println!(
            "  5. dc through the remaining {} sts back up to the top.",
            self.dc_between_tentacles
        );
        println!("  6. sl st to first st, fasten off, weave in ends.");
        println!(
            "     Add a quick {} leaf or stalk to the top loop if desired!\n",
            YarnColor::Green
        );
    }

    fn print_math(&self) {
        println!("PATTERN MATH");
        println!("  Circle sts (each) ........ {}", self.circle_stitch_count());
        println!("  Edge join stitches ....... {}", self.edge_stitch_count());
        println!("  Tentacle loops ........... {}", self.tentacle_count);
        println!("  Beads per tentacle ....... {}", self.beads_per_tentacle);
        println!("  TOTAL beads pre-threaded . {}", self.total_beads());
    }

    // ---------- Edge simulator ----------

    fn simulate_join(&self) {
        println!("\nEDGE TRACE (stitch-by-stitch)");
        let mut idx: u32 = 0;

        // Opening join
        idx += 1;
        self.step(idx, Stitch::DoubleCrochet, "join top through both layers");

        // Run of plain DCs
        for _ in 0..self.dc_between_tentacles {
            idx += 1;
            self.step(idx, Stitch::DoubleCrochet, "edge dc");
        }

        // Tentacle anchors + bead strands
        for t in 1..=self.tentacle_count {
            idx += 1;
            self.step(
                idx,
                Stitch::DoubleCrochet,
                &format!("anchor — tentacle {}", t),
            );

            self.step_chain(10, "chain out");

            for b in 1..=self.beads_per_tentacle {
                idx += 1;
                println!("  [{:>3}]  bead-{}  slide bead up + ch 1 to lock", idx, b);
            }

            self.step_chain(10, "chain back");
            idx += 1;
            self.step(idx, Stitch::SlipStitch, "anchor back into dc");
        }

        // Return run of plain DCs
        for _ in 0..self.dc_between_tentacles {
            idx += 1;
            self.step(idx, Stitch::DoubleCrochet, "return dc");
        }

        // Closing join
        idx += 1;
        self.step(idx, Stitch::SlipStitch, "join to first st, fasten off");
        println!("  ── total counted steps: {} ──", idx);
    }

    fn step(&self, i: u32, s: Stitch, note: &str) {
        let label = match s {
            Stitch::Chain(n) => format!("ch {}", n),
            other => other.abbr().to_string(),
        };
        println!("  [{:>3}]  {:>5}  {}", i, label, note);
    }

    fn step_chain(&self, n: u8, note: &str) {
        let s = Stitch::Chain(n);
        println!("         {:>5}  {}", s.abbr(), format!("{} — ch {}", note, n));
    }

    // ---------- Driver ----------

    fn render(&self) {
        self.print_header();
        self.print_materials();
        self.print_body();
        self.print_prep();
        self.print_tentacles();
        self.print_math();
        self.simulate_join();
    }
}

// ---------- Main ----------

fn main() {
    let octo = Octopus::default();
    octo.render();
    println!("\n✨ zine page ready — print & fold ✨");
}
