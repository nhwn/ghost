use itertools::iproduct;
use wasmer_runtime::{imports, instantiate, Array, Func, Memory, WasmPtr};

const FLAG_LEN: i32 = 13;
static WASM_BYTES: &'static [u8] = include_bytes!("./ghost_bg.wasm");

fn main() {
    let h0 = std::thread::spawn(|| {
        let import_object = imports! {};
        let instance = instantiate(WASM_BYTES, &import_object).unwrap();
        let memory = instance.exports.get::<Memory>("memory").unwrap();
        let malloc = instance
            .exports
            .get::<Func<i32, i32>>("__wbindgen_malloc")
            .unwrap();
        let check = instance
            .exports
            .get::<Func<(i32, i32), i32>>("check")
            .unwrap();
        let iter = iproduct!(
            b'A'..=b'C',
            b'A'..=b'Z',
            b'A'..=b'Z',
            b'A'..=b'Z',
            b'0'..=b'9',
            b'0'..=b'9',
            b'0'..=b'9',
            b'0'..=b'9'
        );
        // try all possible permutations of SKY-[A-Z]{4}-\d{4}
        for (a0, a1, a2, a3, d0, d1, d2, d3) in iter {
            // get buffer in wasm memory
            let raw_ptr = malloc.call(FLAG_LEN).unwrap();
            // Rust wrapper around our malloc'd pointer
            let ptr = WasmPtr::<u8, Array>::new(raw_ptr as u32);
            // SAFETY: we just allocated 13 bytes, so it's safe to index into cells
            unsafe {
                // slice of length 13 of cells (each cell is a byte)
                let cells = ptr.deref_mut(&memory, 0, FLAG_LEN as u32).unwrap();
                // initialize cells
                cells.get_unchecked_mut(0).set(b'S');
                cells.get_unchecked_mut(1).set(b'K');
                cells.get_unchecked_mut(2).set(b'Y');
                cells.get_unchecked_mut(3).set(b'-');
                cells.get_unchecked_mut(4).set(a0);
                cells.get_unchecked_mut(5).set(a1);
                cells.get_unchecked_mut(6).set(a2);
                cells.get_unchecked_mut(7).set(a3);
                cells.get_unchecked_mut(8).set(b'-');
                cells.get_unchecked_mut(9).set(d0);
                cells.get_unchecked_mut(10).set(d1);
                cells.get_unchecked_mut(11).set(d2);
                cells.get_unchecked_mut(12).set(d3);
            }
            if check.call(raw_ptr, FLAG_LEN).unwrap() != 0 {
                println!("{}", ptr.get_utf8_string(&memory, FLAG_LEN as u32).unwrap());
                std::process::exit(0);
            }
        }
    });
    let h1 = std::thread::spawn(|| {
        let import_object = imports! {};
        let instance = instantiate(WASM_BYTES, &import_object).unwrap();
        let memory = instance.exports.get::<Memory>("memory").unwrap();
        let malloc = instance
            .exports
            .get::<Func<i32, i32>>("__wbindgen_malloc")
            .unwrap();
        let check = instance
            .exports
            .get::<Func<(i32, i32), i32>>("check")
            .unwrap();
        let iter = iproduct!(
            b'D'..=b'F',
            b'A'..=b'Z',
            b'A'..=b'Z',
            b'A'..=b'Z',
            b'0'..=b'9',
            b'0'..=b'9',
            b'0'..=b'9',
            b'0'..=b'9'
        );
        // try all possible permutations of SKY-[A-Z]{4}-\d{4}
        for (a0, a1, a2, a3, d0, d1, d2, d3) in iter {
            // get buffer in wasm memory
            let raw_ptr = malloc.call(FLAG_LEN).unwrap();
            // Rust wrapper around our malloc'd pointer
            let ptr = WasmPtr::<u8, Array>::new(raw_ptr as u32);
            // SAFETY: we just allocated 13 bytes, so it's safe to index into cells
            unsafe {
                // slice of length 13 of cells (each cell is a byte)
                let cells = ptr.deref_mut(&memory, 0, FLAG_LEN as u32).unwrap();
                // initialize cells
                cells.get_unchecked_mut(0).set(b'S');
                cells.get_unchecked_mut(1).set(b'K');
                cells.get_unchecked_mut(2).set(b'Y');
                cells.get_unchecked_mut(3).set(b'-');
                cells.get_unchecked_mut(4).set(a0);
                cells.get_unchecked_mut(5).set(a1);
                cells.get_unchecked_mut(6).set(a2);
                cells.get_unchecked_mut(7).set(a3);
                cells.get_unchecked_mut(8).set(b'-');
                cells.get_unchecked_mut(9).set(d0);
                cells.get_unchecked_mut(10).set(d1);
                cells.get_unchecked_mut(11).set(d2);
                cells.get_unchecked_mut(12).set(d3);
            }
            if check.call(raw_ptr, FLAG_LEN).unwrap() != 0 {
                println!("{}", ptr.get_utf8_string(&memory, FLAG_LEN as u32).unwrap());
                std::process::exit(0);
            }
        }
    });
    let h2 = std::thread::spawn(|| {
        let import_object = imports! {};
        let instance = instantiate(WASM_BYTES, &import_object).unwrap();
        let memory = instance.exports.get::<Memory>("memory").unwrap();
        let malloc = instance
            .exports
            .get::<Func<i32, i32>>("__wbindgen_malloc")
            .unwrap();
        let check = instance
            .exports
            .get::<Func<(i32, i32), i32>>("check")
            .unwrap();
        let iter = iproduct!(
            b'G'..=b'I',
            b'A'..=b'Z',
            b'A'..=b'Z',
            b'A'..=b'Z',
            b'0'..=b'9',
            b'0'..=b'9',
            b'0'..=b'9',
            b'0'..=b'9'
        );
        // try all possible permutations of SKY-[A-Z]{4}-\d{4}
        for (a0, a1, a2, a3, d0, d1, d2, d3) in iter {
            // get buffer in wasm memory
            let raw_ptr = malloc.call(FLAG_LEN).unwrap();
            // Rust wrapper around our malloc'd pointer
            let ptr = WasmPtr::<u8, Array>::new(raw_ptr as u32);
            // SAFETY: we just allocated 13 bytes, so it's safe to index into cells
            unsafe {
                // slice of length 13 of cells (each cell is a byte)
                let cells = ptr.deref_mut(&memory, 0, FLAG_LEN as u32).unwrap();
                // initialize cells
                cells.get_unchecked_mut(0).set(b'S');
                cells.get_unchecked_mut(1).set(b'K');
                cells.get_unchecked_mut(2).set(b'Y');
                cells.get_unchecked_mut(3).set(b'-');
                cells.get_unchecked_mut(4).set(a0);
                cells.get_unchecked_mut(5).set(a1);
                cells.get_unchecked_mut(6).set(a2);
                cells.get_unchecked_mut(7).set(a3);
                cells.get_unchecked_mut(8).set(b'-');
                cells.get_unchecked_mut(9).set(d0);
                cells.get_unchecked_mut(10).set(d1);
                cells.get_unchecked_mut(11).set(d2);
                cells.get_unchecked_mut(12).set(d3);
            }
            if check.call(raw_ptr, FLAG_LEN).unwrap() != 0 {
                println!("{}", ptr.get_utf8_string(&memory, FLAG_LEN as u32).unwrap());
                std::process::exit(0);
            }
        }
    });
    let h3 = std::thread::spawn(|| {
        let import_object = imports! {};
        let instance = instantiate(WASM_BYTES, &import_object).unwrap();
        let memory = instance.exports.get::<Memory>("memory").unwrap();
        let malloc = instance
            .exports
            .get::<Func<i32, i32>>("__wbindgen_malloc")
            .unwrap();
        let check = instance
            .exports
            .get::<Func<(i32, i32), i32>>("check")
            .unwrap();
        let iter = iproduct!(
            b'J'..=b'L',
            b'A'..=b'Z',
            b'A'..=b'Z',
            b'A'..=b'Z',
            b'0'..=b'9',
            b'0'..=b'9',
            b'0'..=b'9',
            b'0'..=b'9'
        );
        // try all possible permutations of SKY-[A-Z]{4}-\d{4}
        for (a0, a1, a2, a3, d0, d1, d2, d3) in iter {
            // get buffer in wasm memory
            let raw_ptr = malloc.call(FLAG_LEN).unwrap();
            // Rust wrapper around our malloc'd pointer
            let ptr = WasmPtr::<u8, Array>::new(raw_ptr as u32);
            // SAFETY: we just allocated 13 bytes, so it's safe to index into cells
            unsafe {
                // slice of length 13 of cells (each cell is a byte)
                let cells = ptr.deref_mut(&memory, 0, FLAG_LEN as u32).unwrap();
                // initialize cells
                cells.get_unchecked_mut(0).set(b'S');
                cells.get_unchecked_mut(1).set(b'K');
                cells.get_unchecked_mut(2).set(b'Y');
                cells.get_unchecked_mut(3).set(b'-');
                cells.get_unchecked_mut(4).set(a0);
                cells.get_unchecked_mut(5).set(a1);
                cells.get_unchecked_mut(6).set(a2);
                cells.get_unchecked_mut(7).set(a3);
                cells.get_unchecked_mut(8).set(b'-');
                cells.get_unchecked_mut(9).set(d0);
                cells.get_unchecked_mut(10).set(d1);
                cells.get_unchecked_mut(11).set(d2);
                cells.get_unchecked_mut(12).set(d3);
            }
            if check.call(raw_ptr, FLAG_LEN).unwrap() != 0 {
                println!("{}", ptr.get_utf8_string(&memory, FLAG_LEN as u32).unwrap());
                std::process::exit(0);
            }
        }
    });
    let h4 = std::thread::spawn(|| {
        let import_object = imports! {};
        let instance = instantiate(WASM_BYTES, &import_object).unwrap();
        let memory = instance.exports.get::<Memory>("memory").unwrap();
        let malloc = instance
            .exports
            .get::<Func<i32, i32>>("__wbindgen_malloc")
            .unwrap();
        let check = instance
            .exports
            .get::<Func<(i32, i32), i32>>("check")
            .unwrap();
        let iter = iproduct!(
            b'M'..=b'O',
            b'A'..=b'Z',
            b'A'..=b'Z',
            b'A'..=b'Z',
            b'0'..=b'9',
            b'0'..=b'9',
            b'0'..=b'9',
            b'0'..=b'9'
        );
        // try all possible permutations of SKY-[A-Z]{4}-\d{4}
        for (a0, a1, a2, a3, d0, d1, d2, d3) in iter {
            // get buffer in wasm memory
            let raw_ptr = malloc.call(FLAG_LEN).unwrap();
            // Rust wrapper around our malloc'd pointer
            let ptr = WasmPtr::<u8, Array>::new(raw_ptr as u32);
            // SAFETY: we just allocated 13 bytes, so it's safe to index into cells
            unsafe {
                // slice of length 13 of cells (each cell is a byte)
                let cells = ptr.deref_mut(&memory, 0, FLAG_LEN as u32).unwrap();
                // initialize cells
                cells.get_unchecked_mut(0).set(b'S');
                cells.get_unchecked_mut(1).set(b'K');
                cells.get_unchecked_mut(2).set(b'Y');
                cells.get_unchecked_mut(3).set(b'-');
                cells.get_unchecked_mut(4).set(a0);
                cells.get_unchecked_mut(5).set(a1);
                cells.get_unchecked_mut(6).set(a2);
                cells.get_unchecked_mut(7).set(a3);
                cells.get_unchecked_mut(8).set(b'-');
                cells.get_unchecked_mut(9).set(d0);
                cells.get_unchecked_mut(10).set(d1);
                cells.get_unchecked_mut(11).set(d2);
                cells.get_unchecked_mut(12).set(d3);
            }
            if check.call(raw_ptr, FLAG_LEN).unwrap() != 0 {
                println!("{}", ptr.get_utf8_string(&memory, FLAG_LEN as u32).unwrap());
                std::process::exit(0);
            }
        }
    });
    let h5 = std::thread::spawn(|| {
        let import_object = imports! {};
        let instance = instantiate(WASM_BYTES, &import_object).unwrap();
        let memory = instance.exports.get::<Memory>("memory").unwrap();
        let malloc = instance
            .exports
            .get::<Func<i32, i32>>("__wbindgen_malloc")
            .unwrap();
        let check = instance
            .exports
            .get::<Func<(i32, i32), i32>>("check")
            .unwrap();
        let iter = iproduct!(
            b'P'..=b'R',
            b'A'..=b'Z',
            b'A'..=b'Z',
            b'A'..=b'Z',
            b'0'..=b'9',
            b'0'..=b'9',
            b'0'..=b'9',
            b'0'..=b'9'
        );
        // try all possible permutations of SKY-[A-Z]{4}-\d{4}
        for (a0, a1, a2, a3, d0, d1, d2, d3) in iter {
            // get buffer in wasm memory
            let raw_ptr = malloc.call(FLAG_LEN).unwrap();
            // Rust wrapper around our malloc'd pointer
            let ptr = WasmPtr::<u8, Array>::new(raw_ptr as u32);
            // SAFETY: we just allocated 13 bytes, so it's safe to index into cells
            unsafe {
                // slice of length 13 of cells (each cell is a byte)
                let cells = ptr.deref_mut(&memory, 0, FLAG_LEN as u32).unwrap();
                // initialize cells
                cells.get_unchecked_mut(0).set(b'S');
                cells.get_unchecked_mut(1).set(b'K');
                cells.get_unchecked_mut(2).set(b'Y');
                cells.get_unchecked_mut(3).set(b'-');
                cells.get_unchecked_mut(4).set(a0);
                cells.get_unchecked_mut(5).set(a1);
                cells.get_unchecked_mut(6).set(a2);
                cells.get_unchecked_mut(7).set(a3);
                cells.get_unchecked_mut(8).set(b'-');
                cells.get_unchecked_mut(9).set(d0);
                cells.get_unchecked_mut(10).set(d1);
                cells.get_unchecked_mut(11).set(d2);
                cells.get_unchecked_mut(12).set(d3);
            }
            if check.call(raw_ptr, FLAG_LEN).unwrap() != 0 {
                println!("{}", ptr.get_utf8_string(&memory, FLAG_LEN as u32).unwrap());
                std::process::exit(0);
            }
        }
    });
    let h6 = std::thread::spawn(|| {
        let import_object = imports! {};
        let instance = instantiate(WASM_BYTES, &import_object).unwrap();
        let memory = instance.exports.get::<Memory>("memory").unwrap();
        let malloc = instance
            .exports
            .get::<Func<i32, i32>>("__wbindgen_malloc")
            .unwrap();
        let check = instance
            .exports
            .get::<Func<(i32, i32), i32>>("check")
            .unwrap();
        let iter = iproduct!(
            b'S'..=b'V',
            b'A'..=b'Z',
            b'A'..=b'Z',
            b'A'..=b'Z',
            b'0'..=b'9',
            b'0'..=b'9',
            b'0'..=b'9',
            b'0'..=b'9'
        );
        // try all possible permutations of SKY-[A-Z]{4}-\d{4}
        for (a0, a1, a2, a3, d0, d1, d2, d3) in iter {
            // get buffer in wasm memory
            let raw_ptr = malloc.call(FLAG_LEN).unwrap();
            // Rust wrapper around our malloc'd pointer
            let ptr = WasmPtr::<u8, Array>::new(raw_ptr as u32);
            // SAFETY: we just allocated 13 bytes, so it's safe to index into cells
            unsafe {
                // slice of length 13 of cells (each cell is a byte)
                let cells = ptr.deref_mut(&memory, 0, FLAG_LEN as u32).unwrap();
                // initialize cells
                cells.get_unchecked_mut(0).set(b'S');
                cells.get_unchecked_mut(1).set(b'K');
                cells.get_unchecked_mut(2).set(b'Y');
                cells.get_unchecked_mut(3).set(b'-');
                cells.get_unchecked_mut(4).set(a0);
                cells.get_unchecked_mut(5).set(a1);
                cells.get_unchecked_mut(6).set(a2);
                cells.get_unchecked_mut(7).set(a3);
                cells.get_unchecked_mut(8).set(b'-');
                cells.get_unchecked_mut(9).set(d0);
                cells.get_unchecked_mut(10).set(d1);
                cells.get_unchecked_mut(11).set(d2);
                cells.get_unchecked_mut(12).set(d3);
            }
            if check.call(raw_ptr, FLAG_LEN).unwrap() != 0 {
                println!("{}", ptr.get_utf8_string(&memory, FLAG_LEN as u32).unwrap());
                std::process::exit(0);
            }
        }
    });
    let h7 = std::thread::spawn(|| {
        let import_object = imports! {};
        let instance = instantiate(WASM_BYTES, &import_object).unwrap();
        let memory = instance.exports.get::<Memory>("memory").unwrap();
        let malloc = instance
            .exports
            .get::<Func<i32, i32>>("__wbindgen_malloc")
            .unwrap();
        let check = instance
            .exports
            .get::<Func<(i32, i32), i32>>("check")
            .unwrap();
        let iter = iproduct!(
            b'W'..=b'Z',
            b'A'..=b'Z',
            b'A'..=b'Z',
            b'A'..=b'Z',
            b'0'..=b'9',
            b'0'..=b'9',
            b'0'..=b'9',
            b'0'..=b'9'
        );
        // try all possible permutations of SKY-[A-Z]{4}-\d{4}
        for (a0, a1, a2, a3, d0, d1, d2, d3) in iter {
            // get buffer in wasm memory
            let raw_ptr = malloc.call(FLAG_LEN).unwrap();
            // Rust wrapper around our malloc'd pointer
            let ptr = WasmPtr::<u8, Array>::new(raw_ptr as u32);
            // SAFETY: we just allocated 13 bytes, so it's safe to index into cells
            unsafe {
                // slice of length 13 of cells (each cell is a byte)
                let cells = ptr.deref_mut(&memory, 0, FLAG_LEN as u32).unwrap();
                // initialize cells
                cells.get_unchecked_mut(0).set(b'S');
                cells.get_unchecked_mut(1).set(b'K');
                cells.get_unchecked_mut(2).set(b'Y');
                cells.get_unchecked_mut(3).set(b'-');
                cells.get_unchecked_mut(4).set(a0);
                cells.get_unchecked_mut(5).set(a1);
                cells.get_unchecked_mut(6).set(a2);
                cells.get_unchecked_mut(7).set(a3);
                cells.get_unchecked_mut(8).set(b'-');
                cells.get_unchecked_mut(9).set(d0);
                cells.get_unchecked_mut(10).set(d1);
                cells.get_unchecked_mut(11).set(d2);
                cells.get_unchecked_mut(12).set(d3);
            }
            if check.call(raw_ptr, FLAG_LEN).unwrap() != 0 {
                println!("{}", ptr.get_utf8_string(&memory, FLAG_LEN as u32).unwrap());
                std::process::exit(0);
            }
        }
    });
    h0.join().unwrap();
    h1.join().unwrap();
    h2.join().unwrap();
    h3.join().unwrap();
    h4.join().unwrap();
    h5.join().unwrap();
    h6.join().unwrap();
    h7.join().unwrap();
}
