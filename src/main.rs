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
    // get buffer in wasm memory
    let raw_ptr = malloc.call(FLAG_LEN)?;
    // check takes 2 i32s and returns an i32
    // param1: pointer to buffer in wasm memory that contains input
    // param2: length of the input buffer
    // ret: 0 if failure, nonzero if success (probably 1)
    // decompiled function at line 15278 in ./ghost_bg.c
    let check = instance
        .exports
        .get::<Func<(i32, i32), i32>>("check")?;
    // Rust wrapper around our malloc'd pointer
    let ptr = WasmPtr::<u8, Array>::new(raw_ptr as u32);
    // slice of length 13 of cells (each cell is a byte)
    let cells = ptr.deref(&memory, 0, FLAG_LEN as u32).unwrap();
    // initialize cells with base flag
    cells[0].set(b'S');
    cells[1].set(b'K');
    cells[2].set(b'Y');
    cells[3].set(b'-');
    cells[8].set(b'-');
    // try all possible permutations of SKY-[A-Z]{4}-\d{4}
    for a0 in b'A'..=b'Z' {
        for a1 in b'A'..=b'Z' {
            for a2 in b'A'..=b'Z' {
                for a3 in b'A'..=b'Z' {
                    for d0 in b'0'..=b'9' {
                        for d1 in b'0'..=b'9' {
                            for d2 in b'0'..=b'9' {
                                for d3 in b'0'..=b'9' {
                                    cells[4].set(a0);
                                    cells[5].set(a1);
                                    cells[6].set(a2);
                                    cells[7].set(a3);
                                    cells[9].set(d0);
                                    cells[10].set(d1);
                                    cells[11].set(d2);
                                    cells[12].set(d3);
                                    // FIXME: the call to check fails on the 2nd iteration,
                                    // but raw_ptr never changes
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
