// move_semantics5.rs
// Make me compile without adding, removing, or changing any of the
// lines in `main()`.
// Execute `rustlings hint move_semantics5` for hints :)

fn main() {
    let mut x: u32 = 100;
    let y: &mut &mut u32 = &mut &mut x;
    let z: &mut u32 = *y;
    *z += 1000;
    **y += 100;
    assert_eq!(x, 1200);
}
