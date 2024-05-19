use std::vec;

mod macros;

qmod!(pub test, test2);

qfn!(pub wow, i32, p1/Vec<i32>, p2/bool, {
    if qcmp!(&&, p1[0] == p1[1], p2) {
        return 1;
    }else {
        return -1;
    }
});

pub trait TraitTest {
    fn test();
}
pub trait TraitTest2 {
    fn test();
}

qstruct!([Debug] pub Test<T>;
    pub a:T, 
    b:bool, 
    c:test::A
);

qstruct!([Debug, Clone, Copy] Example; o:u32);

qimpl! {[TraitTest] Example;
    fn test() {
        println!("example")
    }
}

qimpl!{[TraitTest] Test<T>;
    fn test() {

    }
}

pub struct Test2<T> where T: TraitTest, T: TraitTest2 {
    a: T
}

impl<T> Test2<T> where T: TraitTest, T: TraitTest2 {
    
}

fn main() {
    let a = 0;
    let mut list = vec![1,2,3,4];

    let test = qmatch!(a; 1,0);

    qmatch!{test; 
        true => { list.push(5) }, 
        _{ list.push(0) }
    }

    qfor!{item: list;
        *item = 0;
    }
    qfor!{i; 2, list.len(); 
        list[i] = 0;
    }

    wow(vec![1,2], false);

    let test2 = Test{
        a: Example{o:1},
        b: false,
        c: test::A::Test
    };

    qvar!(o[Test<Example>], b:test2.a);

    println!("Hello, world! {test2:?}, {test}");
}
