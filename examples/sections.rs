use inka::program;

fn main() {
    let program = dbg!(program());
    let text_base = dbg!(program.get_section(".text").unwrap().base());

    dbg!(unsafe { text_base.add(1) });
}
