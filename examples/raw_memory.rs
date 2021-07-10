use std::slice;

fn main() {
    let s = "str";

    println!("s.as_ptr() = {:?}", s.as_ptr());
    println!("{{:p}} = {:p}", s);
    let s_data = unsafe {
        *(&s  as *const _ as *const usize)
    };
    println!("s.data = {:p}", s_data as *const u8);

    let bytes: &[u8] = unsafe {
        slice::from_raw_parts(&s as *const _ as *const u8, 4 + 16)
    };
    println!("s.bytes = {:02x?}", bytes);
}
