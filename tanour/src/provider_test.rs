use crate::blockchain_api::MockBlockchainAPI;

use super::*;

#[test]
fn test_read() {
    let mut api = Box::new(MockBlockchainAPI::new());
    api.expect_read_storage().returning(|_, len| {
        let mut d = Vec::new();
        d.resize(len as usize, 0);
        Ok(d)
    });
    let mut provider = ProviderAdaptor::new(api, 5);

    let data = provider.read_storage(3, 12).expect("Reading failed");
    assert_eq!(data, vec![0; 12]);
}

#[test]
fn test_write() {
    let mut api = Box::new(MockBlockchainAPI::new());
    api.expect_read_storage().returning(|_, len| {
        let mut d = Vec::new();
        d.resize(len as usize, 0);
        Ok(d)
    });
    let mut provider = ProviderAdaptor::new(api, 5);

    let data = vec![1, 2, 3];
    provider.write_storage(3, &data).expect("Writing failed");

    let expected = provider.read_storage(3, 3).expect("Reading failed");
    assert_eq!(data, expected);
}
