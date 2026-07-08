// 普通值：5
const normal = 5;

// CPS 包装后的 5：接收续体k，把5传给k
const cpsFive = (k) => k(5);

// 运行CPS：传入最终续体（打印）
cpsFive(x => console.log(x)); // 5



// 数字n的cps
const cpsNum = n => k => k(n);
// 乘法cps
const mulCps = (a, b, k) => k(a * b);
// 减1 cps
const sub1Cps = (x, k) => k(x - 1);

// 计算 5 * 4
cpsNum(5)(n => {
  sub1Cps(n, m => {
    mulCps(n, m, res => {
      console.log(res); // 20
    })
  })
})

// k 是一个函数，用于接收上一步运行的结果

// 对k做文章，将k转化为一个容器，该容器用于接收上一步运行的结果，即 x => 容器
// 这样不断地bind，上一步的容器，和接收上一步运算结果的下一步的容器，就形成了一个cps的链式调用
// 即bind(上一步容器，接收上一步运算结果的下一步的容器)，这样就形成了一个cps的链式调用，将回调转化为了平铺调用

// 第一步：创建接收原始值的容器（接收原始值k(5)）
// 第二步，将该容器的运算结果，传递给下一个容器
// 第三步，将下一个容器的运算结果，继续传递给下一个容器
// 第四步，直到最后一个容器，将结果运算出来

// 第一步：Container(cps原始值)
// 第二步，bind(Container(cps原始值), x => Container(cps(x)))
// 第三步，bind(Container(cps(x)), y => Container(cps(y)))，其中Container(cps(x))为第二步中的Container(cps(x))
// 第四步，直到最后一个容器，将结果运算出来

// **************对k做文章**************，这是重点


class Container {
    constructor(cps) {
        this.cps = cps;
    }
}


// 其中f，为接收上一步运算结果的下一步的容器，即 x => 容器
// 其中f，为接收上一步运算结果，并返回一个容器
// bind绑定上一步的容器，和下一步的“容器”（下一步的容器，只所以打引号，是因为要接收上一步的运算结果，形式为 x => 容器）
function bind(container, f) {
    // 返回一个容器，用于接收最后一个函数 new Container(x => f(x));
    return new Container(final_f => {
        // 先计算上一个容器，先计算上一个cps
        container.k(y => { // y为上一个容器的运算结果，将上一个容器的计算结果传递给下一个容器
            var nextContainer = f(y);
            // 运算最后一个容器的函数
            nextContainer.k(final_f);
        });
    });
}



// 原始的cps函数
// x => k => k(x)
// 单纯的cps
// k => k(x)

// 原始值 5
// 变为cps后
// k => k(5)，k里面的5即为原始值


//不是这样子，Container里面是一个cps，即一个cps容器
// bind(new Container(x => x), x => Container(x + 1))

// bind(
//     bind(new Container(k => k(5)), x => new Container(k => k(x - 1))),
//     x => new Container(k => k(x * 2))
// )


// 每一步，一个bind
bind(new Container(k => k(5)), x => 
    bind(new Container(k => k(x - 1)), 
            y => new Container(k => k(x * y))))
            .k(res => console.log(res)) // 20

// bind(new Container(k => k(5)), x => 
//     bind(new Container(k => k(x - 1)), 
//             y => 
//                 bind(new Container(k => k(x * y)), z => new Container(k => k(z))))) // 最后异步相当于 x => f(x)
//             .k(res => console.log(res)) // 20            


