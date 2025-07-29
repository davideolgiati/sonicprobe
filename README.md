## SonicProbe
##### a cli tool to analyze flac audio files

#### This tool is in an alpha state

it is stable but it may be not very accurate.  
**Testers wanted!**

#### Features:

- [x] Channels Count
- [x] Sample Rate Detection
- [x] Bit Depth Detection
- [x] Channels Balance Analisys
- [x] RMS Analisys
- [x] Peak Analisys
- [x] True Peak Analisys
- [x] Samples clipping Analisys
- [x] Reconstructed Samples clipping Analisys
- [x] Average Sample Value Analisys
- [x] Crest Factor Analisys
- [ ] Noise Floor Analisys
- [ ] Signal to Noise Ratio Analisys
- [ ] Total Harmonic Distortion Analisys
- [ ] Integrated Loudness Analisys

#### Install guide:
 1) clone this repo
 2) install rust compiler on your machine
 3) enter the repo directory
 4) run `cargo install --path .`
 5) done !

#### Usage:

run in your preferred terminal the following command:  
`sonicprobe "path/to/your/file.flac"`