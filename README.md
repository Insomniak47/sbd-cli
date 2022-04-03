# SBD

Making your text block diagrams better.. a little bit.

## What is this?

sbd is a way to translate block diagrams.. commonly done in text format with dashes and angle brackets and pipes into something that doesn't read as awfully.

What does it look like?

```txt
//Before

       ---------- HDMI ----> BENQ 24"   
       |---------- DP  ----> Dell 24"
       |---------- DP  ----> BENQ 27"
       |
Desktop-----USB-C-----> USB-C Extension ---USB-C---> USB-C to 4x USB-A (3.0) hub ---- USB-A ---> Mouse
Laptop-----------USB-C---------------------------------|                          |--- USB-A ---> Keeb
       |                                                                          |--- USB-A ---> Yubi
       |                                                                          ---- USB-A ---> Mic   
       |
       |--------TB4--------> Dock (when it gets here) --- DP -------> DELL 24"
       |                                              |-- DP -------> BENQ 24"
       |                                              |-- USB-A-----> WebCam
       |                                              ----Maybe???--> USB-C to 4x USB-A (3.0) hub??
       |
       ----------HDMI-------> HDMI (Monitor 3)          

after:

│        ┌───────── HDMI ────► BENQ 24"                                                                     │
│        ├────────── DP  ────► Dell 24"                                                                     │
│        ├────────── DP  ────► BENQ 27"                                                                     │
│        │                                                                                                  │
│ Desktop─────USB─C─────► USB─C Extension ───USB─C───► USB─C to 4x USB─A (3.0) hub ──── USB─A ───► Mouse    │
│ Laptop───────────USB─C─────────────────────────────────┘                          ├─── USB─A ───► Keeb    │
│        │                                                                          ├─── USB─A ───► Yubi    │
│        │                                                                          └─── USB─A ───► Mic     │
│        │                                                                                                  │
│        ├────────TB4────────► Dock (when it gets here) ┌── DP ───────► DELL 24"                            │
│        │                                              ├── DP ───────► BENQ 24"                            │
│        │                                              ├── USB─A─────► WebCam                              │
│        │                                              └───Maybe???──► USB─C to 4x USB─A (3.0) hub??       │
│        │                                                                                                  │
│        └─────────HDMI───────► HDMI (Monitor 3)                                                            │
│                                                                                                           │

```

## Limitations

- Probably isn't that fast
- Definitely uses more memory than it needs to
- Doesn't support anything to wacky or things I haven't used really. If you want more added ask away

## FAQ

- My 50MB diagram takes too long to go through SBD
  - If you're making 50MB diagrams in crappy text format you have other problems

- This uses more RAM than it needs to
  - Sure does!
