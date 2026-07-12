// 不要运行：编译报错，Future 绑定非'static引用
#[tokio::main]
async fn main() {
    let data = String::from("hello tokio");

    // ❌ 闭包只是引用捕获 data，Future 生命周期 != 'static
    let task = tokio::spawn(async {
        println!("{}", data);
    });

    task.await.unwrap();
}


// 报错根源：异步 Future 悄悄持有 &String 引用，不满足 'static，无法交给 tokio 后台任务。