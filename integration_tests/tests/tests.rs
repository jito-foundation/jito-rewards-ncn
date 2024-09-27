#[cfg(test)]
mod tests {
    use solana_sdk::signature::{Keypair, Signer};

    #[tokio::test]
    async fn test_ok() {
        assert_eq!(1, 1);
    }
}
