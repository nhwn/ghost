use wasmer_runtime::{
    instantiate,
    Func,
    Memory,
    Array,
    imports,
    error,
    WasmPtr,
};

fn main() -> error::Result<()> {
    const FLAG_LEN: i32 = 13;
    // instantiation boilerplate
    let wasm_bytes = include_bytes!("./ghost_bg.wasm");
    let import_object = imports! {};
    let instance = instantiate(wasm_bytes, &import_object)?;
    let memory = instance.exports.get::<Memory>("memory")?;
    let malloc = instance.exports.get::<Func<i32, i32>>("__wbindgen_malloc")?;
    // check takes 2 i32s and returns an i32
    // param1: pointer to buffer in wasm memory that contains input
    // param2: length of the input buffer
    // ret: 0 if failure, nonzero if success (probably 1)
    // decompiled function at line 15278 in ./ghost_bg.c
    let check = instance
        .exports
        .get::<Func<(i32, i32), i32>>("check")?;

    // try all possible permutations of SKY-[A-Z]{4}-\d{4}
    for a0 in b'A'..=b'Z' {
        for a1 in b'A'..=b'Z' {
            for a2 in b'A'..=b'Z' {
                for a3 in b'A'..=b'Z' {
                    for d0 in b'0'..=b'9' {
                        for d1 in b'0'..=b'9' {
                            for d2 in b'0'..=b'9' {
                                for d3 in b'0'..=b'9' {
                                    // get buffer in wasm memory
                                    let raw_ptr = malloc.call(FLAG_LEN)?;
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
                                    if check.call(raw_ptr, FLAG_LEN)? != 0 {
                                        println!("{}", ptr.get_utf8_string(&memory, FLAG_LEN as u32).unwrap());
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
