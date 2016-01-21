fn main() {
    let m: &u32 = {
        let n = &5u32;
        &*n
    };
    let o = *m;
}
