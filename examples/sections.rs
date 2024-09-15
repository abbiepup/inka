use inka::Hook;

fn main() {
    unsafe extern "C" fn hook_me() {
        println!("Original");
    }

    let _guard = (hook_me as unsafe extern "C" fn()).hook(|| println!("Hooked"));
    dbg!(_guard);

    unsafe { hook_me() };
}
