mod core;

fn main() {
    let t = core::sub1::T1;
    t.take();
    let t = core::sub2::T2;
    t.take();
}
