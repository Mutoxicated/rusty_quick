mod macros;

qmod!(pub test, test2);

qfn!(pub wow, i32, p1/Vec<i32>, p2/bool, {
    if qop!(&&, p1[0] == p1[1], p2) {
        return 1;
    }else {
        return -1;
    }
});

#[derive(Debug)]
#[allow(dead_code)]
pub struct Test {
    a:i32,
    b:bool,
    c:test::A
}

trait TraitTest {
    fn test() {}
}

qimpl!{[TraitTest] Test;
    
}

fn main() {
    let a = 0;
    let mut list = [1,2,3,4];

    let test = qmatch!(a,1,0);

    qfor!{item: list;
        *item = 0;
    }
    qfor!{i; 2, list.len(); 
        list[i] = 0;
    }

    wow(vec![1,2], false);

    let test2 = Test{
        a: 1,
        b: false,
        c: test::A::Test
    };

    qvar!(o[Test], b:test2.a);

    println!("Hello, world! {test2:?}, {test}");
}
