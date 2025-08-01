# 🎵 SonicProbe

**A comprehensive command-line audio analysis tool built in Rust**

SonicProbe is a powerful CLI application designed for audio engineers, producers, and developers who need detailed technical analysis of audio files. Built with Rust for speed and reliability, it provides professional-grade audio metrics and diagnostics.

## ✨ Features


### 📊 File & Format Analysis
- **File Information** - Size, duration, sample count, and format detection
- **Bit Depth Analysis** - Actual bit depth usage detection and range analysis
- **Sample Rate Detection** - Precise sample rate identification

### 🎛️ Stereo Field Analysis
- **Channel Balance** - RMS balance analysis between left/right channels
- **Stereo Correlation** - Measures stereo width and phase relationships
- **Per-Channel Metrics** - Individual left/right channel analysis

### 📈 Dynamic Range & Levels
- **RMS Level** - Root Mean Square level measurement per channel
- **Peak Level** - Maximum sample peak detection
- **True Peak** - Inter-sample peak detection for broadcast compliance
- **Crest Factor** - Dynamic range measurement (peak-to-RMS ratio)
- **Dynamic Range (DR)** - Industry-standard dynamic range measurement

### 🔍 Signal Quality Analysis
- **DC Offset Detection** - Identifies unwanted DC bias in audio
- **Zero Crossing Rate** - Measures signal complexity and noise characteristics
- **Clipping Detection** - Sample-level clipping analysis
- **True Clipping** - Inter-sample clipping detection for digital artifacts

## 🚀 Quick Start

### Installation

1. **Install Rust** (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Clone and Install SonicProbe**:
   ```bash
   git clone https://github.com/davideolgiati/sonicprobe.git
   cd sonicprobe
   cargo install --path .
   ```

### Usage

Analyze any supported audio file:

```bash
sonicprobe "path/to/your/audio/file.flac"
```

**Supported formats**: FLAC

## 📖 Example Output

```
======================================================================
                  SONICPROBE - AUDIO ANALYSIS REPORT                  
======================================================================

── FILE DETAILS ──────────────────────────────────────────────────────

   Filename           : 03 - giorgio by moroder
   Size               : 42.9 MB
   Sample Count       : 24013424
   Duration           : 09:05
   Format             : 16  bit / 44100   Hz
   Bit depth usage    : 15 bit (Range 0-16)


── STEREO FIELD ANALYSIS ─────────────────────────────────────────────

   Channels           :  2
   RMS Balance (L/R)  : +0.02   dB
   Stereo Correlation :  0.79


┌──────────────────────────┬────────────────────┬────────────────────┐
│  CHANNEL ANALYSIS        │              LEFT  │             RIGHT  │
├──────────────────────────┼────────────────────┼────────────────────┤
│  RMS Level               │       -26.80   dB  │       -26.81   dB  │
│  Peak Level              │        -8.31   dB  │        -8.55   dB  │
│  True Peak               │        -8.31   dB  │        -8.46   dB  │
│  Crest Factor            │       +18.49   dB  │       +18.26   dB  │
│  DC Offset               │     -0.00162    V  │     -0.00168    V  │
│  Zero Crossing Rate      │         1676   Hz  │         1886   Hz  │
│  Dynamic Range           │           17   DR  │           16   DR  │
├──────────────────────────┼────────────────────┼────────────────────┤
│  Clipping                │      0.00000    %  │      0.00000    %  │
│  True Clipping           │      0.00000    %  │      0.00000    %  │
└──────────────────────────┴────────────────────┴────────────────────┘
```

## 🛠️ Development Status

**Current Status**: Beta - Stable but may have accuracy limitations  
**Seeking**: Beta testers and feedback from audio professionals

### Roadmap
- [ ] Additional audio format support
- [ ] Batch processing capabilities  
- [ ] CSV output formats

## 🤝 Contributing

We welcome contributions! SonicProbe is actively seeking:

- **Beta Testers** - Try it with your audio files and report issues
- **Audio Engineers** - Validate measurement accuracy against reference tools
- **Developers** - Code contributions, optimizations, and new features

### Getting Started with Development

```bash
# Clone the repository
git clone https://github.com/davideolgiati/sonicprobe.git
cd sonicprobe

# Run tests
cargo test

# Run with debug output
cargo run -- "test_file.flac"
```

## 🎯 Use Cases

- **Audio Mastering** - Quality control and loudness compliance
- **Podcast Production** - Ensuring consistent audio levels
- **Music Production** - Analyzing mix dynamics and headroom
- **Audio Forensics** - Detecting compression artifacts and quality issues
- **Archival Work** - Assessing digitized audio quality

## 🔧 Technical Details

- **Language**: Rust 🦀
- **Architecture**: Multi-threaded CLI with efficient memory usage
- **Dependencies**: flac, rayon
- **Accuracy**: Professional-grade algorithms (beta accuracy validation ongoing)

## 📋 Requirements

- **Rust**: 1.70+ (latest stable recommended)
- **OS**: Linux, macOS, Windows

## Support & Feedback

- **Issues**: [GitHub Issues](https://github.com/davideolgiati/sonicprobe/issues)
- **Discussions**: [GitHub Discussions](https://github.com/davideolgiati/sonicprobe/discussions)
- **Feature Requests**: Open an issue with the `enhancement` label
