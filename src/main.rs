mod macro_tests;

qmod!(/pub test, test2);

qfn!(wow, i32, p1/Vec<usize>, p2/bool, {
    if qop!(&&, p1[0] == p1[1], p2) {
        return 1;
    }else {
        return -1;
    }
});

qfn!(example_function, String, p1/bool, {
    if p1 {
        return String::from("true");
    }else {
        return String::from("false");
    }
});

qenum!([Debug] TestEnum; Three(i32), Four, Five, Six);

qenum!(TestEnum2; One);

#[derive(Debug)]
#[allow(dead_code)]
pub struct Test {
    a:i32,
    b:bool,
    c:TestEnum
}

fn main() {
    let a = 0;
    let mut list = [1,2,3,4];

    let test = qmatch!(a,1,0);

    qfor!(item, list, {
        *item = 0;
    });
    qfor!(i / list.len(), {
        list[i] = 0;
    });

    wow(vec![1,2], false);

    let test2 = Test{
        a: 1,
        b: false,
        c: TestEnum::Four
    };

    println!("Hello, world! {test2:?}, {test}");
}
