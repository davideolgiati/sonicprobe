# SonicProbe

**Easy audio analysis in the command line**

SonicProbe is a powerful standalone command line utility designed for audio engineers, producers and developers looking for detailed audio files analysis.

**Currenty the only available format is FLAC**

```
======================================================================
                  SONICPROBE - AUDIO ANALYSIS REPORT                  
======================================================================

── FILE DETAILS ──────────────────────────────────────────────────────

   Filename           : test1
   Size               : 13 MB
   Sample Count       : 14974008
   Duration           : 02:49
   Sample Rate        : 44.1 kHz - Standard for consumer audio
   Bit Depth          : 16  bit - CD standard
   Bit depth usage    : 16  bit



── STEREO FIELD ANALYSIS ─────────────────────────────────────────────

   Channels           : 2
   RMS Balance (L/R)  : +1.44   dB
   Stereo Correlation : +12.95163    %



┌──────────────────────────┬────────────────────┬────────────────────┐
│  CHANNEL ANALYSIS        │              LEFT  │             RIGHT  │
├──────────────────────────┼────────────────────┼────────────────────┤
│  RMS Level               │       -16.90   dB  │       -18.33   dB  │
│  Peak Level              │        -0.18   dB  │        -0.19   dB  │
│  True Peak               │        -0.17   dB  │        -0.17   dB  │
│  Crest Factor            │       +16.72   dB  │       +18.14   dB  │
│  DC Offset               │     +0.00001    V  │     -0.00000    V  │
│  Zero Crossing Rate      │         1685   Hz  │         1658   Hz  │
│  Dynamic Range           │           19   DR  │           19   DR  │
├──────────────────────────┼────────────────────┼────────────────────┤
│  Clipping                │      0.00000    %  │      0.00000    %  │
│  True Clipping           │      0.00000    %  │      0.00000    %  │
└──────────────────────────┴────────────────────┴────────────────────┘
```

## Quick Start

### Installation

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone and Install SonicProbe**:
   ```bash
   git clone https://github.com/davideolgiati/sonicprobe.git
   cd sonicprobe
   cargo install --path sonicprobe-cli
   ```

   The repository root is a Cargo workspace, so the install target is the
   `sonicprobe-cli` crate (not `.`). This builds the `sonicprobe` binary in
   release mode and places it in `~/.cargo/bin/` — make sure that directory is
   on your `PATH`. Re-run with `--force` to overwrite an existing install.

### Usage

Analyze any supported audio file:

```bash
sonicprobe "path/to/your/audio/file.flac"
```

Emit machine-readable JSON instead of the plain-text report:

```bash
sonicprobe "path/to/your/audio/file.flac" --json
```

## Contributing

SonicProbe is actively seeking:

- **Beta Testers**
- **Developers**

## Requirements

- **Rust**: 1.88+ (latest stable recommended) — uses `slice::as_chunks`, stabilized in 1.88
