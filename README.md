# Planszówkowy Player

Tutaj znajdziesz całą dokumentacje i wszystkie pliki które powstną podczas tworzenia zabawki wymyślonej któregoś wieczoru grając w planszówki.

# [Planszówkowy player](Planszówkowy-player.md)

Pierwszy szkic zamysłu oraz założeń które nasze użądzenia będzie musiało spełniać

# Stack

- Hardware -> ESP32
- Embedded software -> esp-idf
- Web application -> Next.js

# Build instruction

Before trying to build esprs code first you need to make sure you have all the **requirements** installed. I recommend following steps **3.1 to 3.5** of [This great esp-rs guide.org guide](https://docs.esp-rs.org/book/installation/index.html).

## Build

If you are on a Windows machine you need to copy esprs to your PC root directory

```
cp -r ./esprs /c/
```

Make sure you are in the repository root when executing this command
After that, it's as simple as connecting your eps32 and running\

```
cargo build
cargo run
```

> First `cargo build` usually takes around 8 minutes, be patient.
