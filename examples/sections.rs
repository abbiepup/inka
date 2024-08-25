use inka::program;

fn main() {
    program().sections().iter().for_each(|section| {
        dbg!(section);
    });
}
