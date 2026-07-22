/// 仅在 feature = "json" 启用时编译
#[cfg(feature = "json")]
use serde::Serialize;

#[cfg(feature = "json")]
#[derive(Debug, Serialize)]
pub struct User {
    pub name: String,
    pub age: u32,
}

#[cfg(feature = "json")]
impl User {
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
}

/// 仅在 feature = "async" 启用时编译
#[cfg(feature = "async")]
pub async fn async_hello() -> String {
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    "Hello from async tokio!".to_string()
}

/// 始终存在的基础函数
pub fn base_hello() -> &'static str {
    "Basic function, no feature required"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_base() {
        assert_eq!(base_hello(), "Basic function, no feature required");
    }

    #[cfg(feature = "json")]
    #[test]
    fn test_json() {
        let u = User {
            name: "alice".into(),
            age: 20,
        };
        let s = u.to_json().unwrap();
        println!("json str: {s}");
    }

    #[cfg(feature = "async")]
    #[tokio::test]
    async fn test_async() {
        let res = async_hello().await;
        println!("{}", res);
    }
}
