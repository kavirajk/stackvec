# StackVec
A normal `Vec`, except storage is backed by static array.

Doesn't need any memory allocator(as backed by static array). And no `std` used. Useful for bare metal programming!

Created as a part of [cs140e](https://cs140e.sergio.bz/assignments/1-shell/#subphase-a-stackvec)

# Usage

```rust
    let mut storage = [0u8; 1024];
    let mut vec = StackVec::new(&mut storage);

    for i in 0..10 {
        vec.push(i * i).expect("can push 1024 times");
    }

    for (i, v) in vec.iter().enumerate() {
        assert_eq!(*v, (i * i) as u8);
    }

    let last_element = vec.pop().expect("has elements");
    assert_eq!(last_element, 9 * 9);

```

# Licence

MIT
