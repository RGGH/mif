[![Rust](https://github.com/RGGH/mif/actions/workflows/rust.yml/badge.svg)](https://github.com/RGGH/mif/actions/workflows/rust.yml)[![Rust Cross Compile for Raspberry Pi 5 (ARM64)](https://github.com/RGGH/mif/actions/workflows/cross_comp_pi5.yaml/badge.svg)](https://github.com/RGGH/mif/actions/workflows/cross_comp_pi5.yaml)

# Adventures in frame buffers - minifb

### Features
- [x] image to bytes 
- [x] collision detection
- [x] frames update from image using slice unless game logic says otherwise
- [x] collision detection for each drop to hit cat

  - Positive score: Original background.
  - Score 0 to -10: Mouse1 background.
  - Score -11 to -20: Mouse2 background.
  - Score below -20: Mono background.


![image](https://github.com/user-attachments/assets/841e855f-37fc-4941-a779-e6fe92fbdd1c)
