use rand::seq::SliceRandom;

struct Card {
    title: &'static str,
    theme: &'static str,
    message: &'static str,
    art: &'static str,
}

const DECK: [Card; 20] = [
    Card {
        title: "THE BAT",
        theme: "Change direction quickly",
        message: "Change direction quickly without needing to explain yourself.",
        art: r#"
   /\_/\
  ( o.o )   /\___/\
   > ^ <   /       \
  /  |  \ /  /| |\  \
"#
    },
    Card {
        title: "THE PUMPKIN",
        theme: "Abundance",
        message: "Make something bold...",
        art: r#"
       ___
     /  _  \
    |  / \  |
   /|  |_|  |\
  ( |  ___  | )
   \| (___) |/
     \_____/
"#
    },
    Card {
        title: "THE COBWEB",
        theme: "Connection",
        message: "Pay attention to the subtle...",
        art: r#"
     \  |  /
   ---\-+-/---
     /  |  \
   -/---|---\-
     /  |  \
"#
    },
    Card {
        title: "THE SKELETON",
        theme: "Structure",
        message: "Strip away the excess...",
        art: r#"
      .-.
     (o.o)
      |=|
     /| |\
    (_|_|_)
     |   |
"#
    },
    Card {
        title: "THE GHOST",
        theme: "Memory",
        message: "Unfinished business will...",
        art: r#"
     .-.
    ( o o )
    |  O  |
    |     |
   /  /\   \
  (~~/  \~~~)
"#
    },
    Card {
        title: "THE CANDLE",
        theme: "Focus",
        message: "You don't need to see...",
        art: r#"
       (
      ) )
     ( (
      ||
     .--.
     |  |
     |  |
    '----'
"#
    },
    Card {
        title: "THE WITCH",
        theme: "Agency",
        message: "You hold the authority...",
        art: r#"
       /\
      /  \
     /____\
   (________)
    /  o o  \
   (   ==   )
"#
    },
    Card {
        title: "THE CAULDRON",
        theme: "Alchemy",
        message: "Let disparate elements sit...",
        art: r#"
     (  )  (
    (  )  ) )
   .----------.
  (            )
  |            |
  \            /
   `----------'
     /      \
"#
    },
    Card {
        title: "THE SPELLS",
        theme: "Intention",
        message: "Words carry weight—be precise...",
        art: r#"
    /  \      *
   / *  \    / \
  /______\  /___\
  |  *   |  |   |
  |______|  |___|
"#
    },
    Card {
        title: "THE TOMBSTONE",
        theme: "Closure",
        message: "Something can be completely...",
        art: r#"
     .----.
    /      \
   |  R.I.P |
   |        |
   |        |
  _|________|_
"#
    },
    Card {
        title: "THE VAMPIRE",
        theme: "Boundaries",
        message: "Stop giving energy to...",
        art: r#"
    /\_____/\
   /  v   v  \
  (   o   o   )
   \    ^    /
    \  ===  /
     `-----'
"#
    },
    Card {
        title: "THE SPIDER",
        theme: "Patience",
        message: "Sit quietly and let...",
        art: r#"
    /\  /\
   //\\//\\
  / \/  \/ \
    (o.o)
   / \  / \
  //\\//\\
"#
    },
    Card {
        title: "THE CROW",
        theme: "Perspective",
        message: "Observe from above...",
        art: r#"
     .-.
    (   )>
    /| |\
   (_|_|_)
    // \\
"#
    },
    Card {
        title: "THE MOTH",
        theme: "Attraction",
        message: "Mind what draws your focus in the dark.",
        art: r#"
    \  /
   /\\//\\
  (  o  )
   \//\\/
    /  \
"#
    },
    Card {
        title: "THE MOON",
        theme: "Instinct",
        message: "Trust what you know...",
        art: r#"
     .-''''-.
    /  .-.   \
   /  /   \   \
  |  |     |   |
   \  \   /   /
    \  `-'   /
     `-....-'
"#
    },
    Card {
        title: "THE HAUNTED HOUSE",
        theme: "Interiority",
        message: "Inspect the rooms inside your mind that you usually leave locked.",
        art: r#"
       /\
      /  \
     /____\
    |  _   |
    | | |  |
    |_|_|__|
"#
    },
    Card {
        title: "THE BLACK CAT",
        theme: "Autonomy",
        message: "Walk your own path...",
        art: r#"
   /\_/\
  ( o.o )
   > ^ <
  /     \
 (       )
  `-----'
"#
    },
    Card {
        title: "THE BROOMSTICK",
        theme: "Clearance",
        message: "Sweep away the clutter...",
        art: r#"
        /
       /
      /
     /
   /|||\
  //|||\\
"#
    },
    Card {
        title: "THE TOADSTOOL",
        theme: "Hidden growth",
        message: "Vital growth is happening beneath the surface where no one can see.",
        art: r#"
     .-'""'-.
    /   o    \
   (  o    o  )
    '-.____.-'
       |  |
      /____\
"#
    },
    Card {
        title: "THE CRYSTAL BALL",
        theme: "Clarity",
        message: "The answer is already...",
        art: r#"
     .---.
    /     \
   |   *   |
    \     /
   .-'---'-.
  (_________)
"#
    },
];

fn main() {
    let mut rng = rand::thread_rng();

    if let Some(card) = DECK.choose(&mut rng) {
        let theme_str = format!("[ {} ]", card.theme);

        println!("┌────────────────────────────────────────────────────────┐");
        println!("│ {:^54} │", card.title);
        println!("├────────────────────────────────────────────────────────┤");
        println!("│ {:^54} │", theme_str);
        println!("│                                                        │");
        for line in card.art.lines().skip(1) {
            println!("│ {:^54} │", line);
        }
        println!("│                                                        │");
        println!("│ {:^54} │", card.message);
        println!("└────────────────────────────────────────────────────────┘");
    }
}
