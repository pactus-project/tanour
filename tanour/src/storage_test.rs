#[cfg(test)]
mod tests {
    use crate::{
        provider::provider_test::ProviderMock,
        storage::Storage,
        utils,
    };

    #[test]
    fn test_read_write() {
        let address = utils::address_from_hex("deadbeef00000000000000000000000000000000");
        let provider = ProviderMock::new(1024);
        let mut storage = Storage::new(provider, address, 5);

        let data = storage.read_storage(3, 12).expect("Reading failed");
        assert_eq!(data, vec![0;12]);
    }
}
