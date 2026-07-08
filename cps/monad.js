
// 构造一个包含一个函数的容器

// Cont 容器：保存 cps 函数 (k) => finalResult
// 该容器中包含一个函数，
class Cont {
    // 构造函数
  constructor(runCPS) {
    this.run = runCPS; // run = (k) => R
  }

  // 执行容器，传入最终续体k，拿到最终结果
  runCont(k) {
    // run为一个函数，
    // 给该函数传递参数，并返回运行结果
    return this.run(k);
  }
}

// cps 容器
// pure：普通值 a 转 Cont a
// pure x = Cont(k => k(x))
// 用于接收上一个容器的运行结果
const pure = a => new Cont(k => k(a));

// bind：对应 Haskell >>=
// 逻辑：
// 1. 先跑当前cont，拿到值x
// 2. 把x传给f，得到下一个Cont
// 3. 用外层续体k去跑下一个Cont
// const bind = (cont, f) => {
//     //返回一个Cont容器
//     // 容器里面为一个cps
//     // 为什么要返回一个cps容器，因为bind的逻辑是先跑当前cont，拿到值x，再把x传给f，得到下一个Cont，再用外层续体k去跑下一个Cont
//   return new Cont(k => {
//     // 先计算上一个容器，先计算上一个cps
//     cont.run(x => {
//       const nextCont = f(x); // f: a => Cont b
//       // 再计算下一个cps
//       nextCont.run(k);
//     })
//   })
// };

const bind = (cont, pure) => {
    //返回一个Cont容器
    // 容器里面为一个cps
    // 为什么要返回一个cps容器，因为bind的逻辑是先跑当前cont，拿到值x，再把x传给f，得到下一个Cont，再用外层续体k去跑下一个Cont
    // 返回一个容器，用于接收最后一个函数
  return new Cont(k => {
    // 先计算上一个容器，先计算上一个cps
    cont.run(x => {
    
    // pure为接收上一个容器的运算结果，并返回一个新的容器
      const nextCont = pure(x); // f: a => Cont b
      // 再计算下一个cps
      // k为最后一个容器的函数
      nextCont.run(k); // k = x => f(x)
    })
  })
};

// 如果是这样情况，就又成了cps不断嵌套了
// cont.run(x => {
//     nextCont.run(x)
// })
// 问题是nextCont从哪里来？
// 它是从f(x)中返回的，f(x)是一个Cont容器，所以nextCont就是f(x)
// 即bind为
// bind(cont, x => new Cont(k => k(x)))
// bind(容器，x => 容器)
// x => 容器，用于接收上一个容器运行的结果

//cps
// const sub1 = (x, k) => k(x -1);
// 将上面的cps包装为容器
// 减法：接收数字，返回 Cont
// const sub1 = x => new Cont(k => k(x -1));
const sub1 = x => pure(x - 1);
// 乘法：接收两个数字，返回 Cont
const mul = (a, b) => pure(a * b);


// 第一步，构建Cont, pure(5)返回一个Cont容器
// 计算 5 * 4
let computation = bind(pure(5), n => {
  return bind(sub1(n), m => mul(n, m));
});

// let computation =
// bind(pure(5), x => {
//     return bind(sub1(x), y => {
//         return mul(x, y);
//     })
// })

// 执行，最终续体打印结果
computation.runCont(res => console.log(res)); // 20
computation.run(res => console.log(res));