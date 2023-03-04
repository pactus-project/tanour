use std::vec;

use crate::blockchain_api::MockBlockchainAPI;

use super::*;

#[test]
fn test_read() {
    let mut api = Box::new(MockBlockchainAPI::new());
    api.expect_page_size().returning(|| Ok(256));
    api.expect_read_page().returning(|_| Ok(vec![0; 256]));
    let mut provider = ProviderAdaptor::new(api).unwrap();

    let data = provider.read_storage(3, 12).expect("Reading failed");
    assert_eq!(data, vec![0; 12]);
}

#[test]
fn test_write() {
    let mut api = Box::new(MockBlockchainAPI::new());
    api.expect_page_size().returning(|| Ok(256));
    api.expect_read_page().returning(|_| Ok(vec![0; 256]));
    let mut provider = ProviderAdaptor::new(api).unwrap();

    let data = vec![1, 2, 3];
    provider.write_storage(3, &data).expect("Writing failed");

    let expected = provider.read_storage(3, 3).expect("Reading failed");
    assert_eq!(data, expected);
}
