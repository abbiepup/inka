use inka::program;

#[no_mangle]
#[export_name = "test_c"]
#[inline(never)]
pub extern "C" fn test() {
    println!("Hi");
}

fn main() {
    dbg!(program());

    test();
}
