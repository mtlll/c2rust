fn with_params(mut unused: i32, mut used: i32, mut unused2: i32, mut used_immut: i32) {
    let mut unused3 = 1;
    let mut used2 = 2;
    used += used_immut + used2;
}

fn used_in_local(mut p1: i32, mut p2: i32) {
    let mut unused = p1;
    p2 = p1;

    let mut p2 = 1;
    let mut arr = [p2 + 1];
}

struct Foo;

impl Foo {
    fn foo(mut p1: i32, mut p2: i32, mut p3: i32) -> i32 {
        p3 += 1;
        p1
    }
}
