fn main() {
    println!("{}", feature_demo::base_hello());

    #[cfg(feature = "json")]
    {
        let user = feature_demo::User {
            name: "test".to_string(),
            age: 18,
        };
        println!("{}", user.to_json().unwrap());
    }

    #[cfg(feature = "async")]
    {
        // 需要 tokio runtime
        let rt = tokio::runtime::Runtime::new().unwrap();
        let msg = rt.block_on(feature_demo::async_hello());
        println!("{}", msg);
    }
}
