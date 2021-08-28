#[cfg(test)]
mod tests {
    use crate::{
        provider_api::provider_mock::ProviderMock,
        state::State,
        utils,
    };

    #[test]
    fn test_read() {
        let address = utils::address_from_hex("deadbeef00000000000000000000000000000000");
        let provider = ProviderMock::new(1024);
        let mut state = State::new(provider, address, 5);

        let data = state.read_storage(3, 12).expect("Reading failed");
        assert_eq!(data, vec![0;12]);
    }
}
