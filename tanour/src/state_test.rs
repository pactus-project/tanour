use crate::{
    provider_mock::ProviderMock,
    state::{State, StateTrait},
};

#[test]
fn test_read() {
    let provider = ProviderMock::new(1024);
    let mut state = State::new(provider, 5);

    let data = state.read_storage(3, 12).expect("Reading failed");
    assert_eq!(data, vec![0; 12]);
}

#[test]
fn test_write() {
    let provider = ProviderMock::new(1024);
    let mut state = State::new(provider, 5);

    let data = vec![1, 2, 3];
    state.write_storage(3, &data).expect("Reading failed");

    let expected = state.read_storage(3, 3).expect("Reading failed");
    assert_eq!(data, expected);
}
