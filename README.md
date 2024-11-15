[![Rust](https://github.com/RGGH/mif/actions/workflows/rust.yml/badge.svg)](https://github.com/RGGH/mif/actions/workflows/rust.yml)[![Rust Cross Compile for Raspberry Pi 5 (ARM64)](https://github.com/RGGH/mif/actions/workflows/cross_comp_pi5.yaml/badge.svg)](https://github.com/RGGH/mif/actions/workflows/cross_comp_pi5.yaml)

# Adventures in frame buffers - minifb

### Features
- [x] image to bytes 
- [x] frames update from image using slice unless game logic says otherwise
- [x] collision detection for each drop to hit cat (negative) and cursor (positive)

  - Positive score: Original background.
  - Score 0 to -10: Mouse1 background.
  - Score -11 to -20: Mouse2 background.
  - Score below -20: Mono background.

![image](https://github.com/user-attachments/assets/c27a47a8-7f4f-43b2-b6e5-c3b09db23fde)

