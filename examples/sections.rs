use inka::program;

fn main() {
    let program = dbg!(program());
    let text = dbg!(program.get_section(".text").unwrap());

    dbg!(unsafe { text.add(1) });
}
