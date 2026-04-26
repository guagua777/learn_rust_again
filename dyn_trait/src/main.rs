// 定义特征
// Animal 特征本身 = DST 动态大小类型，大小未知，不能直接栈分配；
trait Animal {
    fn sound(&self);
}

// 不同实现类型
// 不同类型，大小不一样，所以，编译期，不知道占用多大空间，所以必须指针化，才能固定大小
// 即使用，动态大小类型，才能固定大小
struct Cat {
    name: String,
    age: u8,
}
struct Dog {
    high: u8,
}
struct Duck {
    name: String,
    length: u32,
    width: u32,
}

impl Animal for Cat {
    fn sound(&self) {
        println!("猫咪：喵喵喵");
    }
}

impl Animal for Dog {
    fn sound(&self) {
        println!("狗狗：汪汪汪");
    }
}

impl Animal for Duck {
    fn sound(&self) {
        println!("鸭子：嘎嘎嘎");
    }
}

// ======================
// 方式1：静态分发 泛型 T: Animal
// 编译期单态化，无运行时开销
// ======================
fn static_talk<T: Animal>(animal: T) {
    animal.sound();
}

// ======================
// 方式2：动态分发 &dyn Animal 引用
// ======================
fn dyn_talk(animal: &dyn Animal) {
    animal.sound();
}

fn main() {
    println!("===== 静态分发 泛型 =====");
    static_talk(Cat {
        name: "小白".to_string(),
        age: 1,
    });
    static_talk(Dog {
        high: 100,
    });

    println!("\n===== 动态分发 &dyn Trait =====");
    let cat = Cat {
        name: "小白".to_string(),
        age: 1,
    };
    let dog = Dog {
        high: 100,
    };
    let duck = Duck {
        name: "小鸭".to_string(),
        length: 100,
        width: 100,
    };
    dyn_talk(&cat);
    dyn_talk(&dog);
    dyn_talk(&duck);

    // 必须得指针化，才可以，这样才能固定大小
    // ======================
    // 核心优势：异构集合 Vec<Box<dyn Animal>>
    // 可以存放完全不同的类型，只有 dyn 能做到
    // ======================
    println!("\n===== 异构集合 Box<dyn Trait> =====");
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Cat {
            name: "小白".to_string(),
            age: 1,
        }),
        Box::new(Dog {
            high: 100,
        }),
        Box::new(Duck {
            name: "小鸭".to_string(),
            length: 100,
            width: 100,
        }),
    ];

    for a in animals {
        a.sound();
    }
}