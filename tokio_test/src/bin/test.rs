pub fn add(a: u64, b: u64) -> u64 {
    a + b
}


async fn double(a: u64) -> u64 {
    a * 2
}


fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }


    #[tokio::test]
    async fn test_double() {
        assert_eq!(double(2).await, 4);
    }

}