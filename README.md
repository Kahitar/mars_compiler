# Build a compiler in Rust

Based on the [this tutorial](https://www.youtube.com/watch?v=vcSijrRsrY0&list=PLUDlas_Zy_qC7c5tCgTMYq2idyyT241qs&ab_channel=Pixeled) writing a compiler in C++.

The initial version (as of 23.08.2023) was done as a learning project with the help of ChatGPT for feedback on code and tips ([Chat](https://chat.openai.com/share/8d72f44f-4866-4a69-8c82-78f2e04212c5)).

## Requirements

- [nasm](https://nasm.us/) 
- [ld - The GNU linker](https://www.man7.org/linux/man-pages/man1/ld.1.html)
    - pre-installed with linux. So probably run this project in WSL

## Basics

- Compile assembly with nasm:

        $ nasm -felf64 test.asm
        
- Link with ld:

        $ ld test.o -o test
        
- Execute:

        $ ./test

- Display exit code:

        $ echo $?